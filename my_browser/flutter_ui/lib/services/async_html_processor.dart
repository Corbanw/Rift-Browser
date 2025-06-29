import 'dart:async';
import 'dart:isolate';
import 'dart:convert';
import '../utils/logger.dart';
import '../utils/html_chunker.dart';
import '../engine_bridge.dart';
import '../models/layout_box.dart';

class ProcessingProgress {
  final double progress;
  final String message;
  final Map<String, dynamic>? data;
  
  ProcessingProgress({
    required this.progress,
    required this.message,
    this.data,
  });
}

class ProcessingResult {
  final List<LayoutBox> layoutBoxes;
  final bool success;
  final String? error;
  final Map<String, dynamic> performanceData;
  
  ProcessingResult({
    required this.layoutBoxes,
    required this.success,
    this.error,
    required this.performanceData,
  });
}

class AsyncHtmlProcessor {
  static final Logger _logger = Logger();
  
  // Processing configuration
  static const int maxProcessingTimeMs = 30000; // 30 seconds max for complex pages
  static const int maxHtmlSizeForChunking = 200000; // 200KB threshold for chunking
  static const int maxHtmlSizeOverall = 2000000; // 2MB absolute max
  static const int maxLayoutBoxes = 100; // Max layout boxes to return
  
  /// Processes HTML content asynchronously with progress callbacks
  static Future<ProcessingResult> processHtmlAsync(
    String html,
    {
      Function(ProcessingProgress)? onProgress,
      bool enableChunking = true,
      int? customChunkSize,
    }
  ) async {
    final stopwatch = Stopwatch()..start();
    final startTime = DateTime.now();
    
    try {
      _logger.info('AsyncProcessor', 'Starting HTML processing', {
        'html_length': html.length,
        'enable_chunking': enableChunking,
        'custom_chunk_size': customChunkSize,
      });
      
      onProgress?.call(ProcessingProgress(
        progress: 0.0,
        message: 'Validating HTML content...',
      ));
      
      // Step 1: Validate and preprocess HTML
      final validationResult = await _validateAndPreprocessHtml(html);
      if (!validationResult.isValid) {
        stopwatch.stop();
        return ProcessingResult(
          layoutBoxes: [],
          success: false,
          error: validationResult.error,
          performanceData: _buildPerformanceData(stopwatch, startTime, 0),
        );
      }
      
      onProgress?.call(ProcessingProgress(
        progress: 0.1,
        message: 'HTML validation completed',
        data: {'html_size': validationResult.processedHtml.length},
      ));
      
      // Step 2: Decide processing strategy
      final shouldUseChunking = enableChunking && 
          validationResult.processedHtml.length > maxHtmlSizeForChunking;
      
      List<LayoutBox> layoutBoxes;
      if (shouldUseChunking) {
        layoutBoxes = await _processWithChunking(
          validationResult.processedHtml,
          onProgress,
          customChunkSize,
        );
      } else {
        layoutBoxes = await _processSinglePass(
          validationResult.processedHtml,
          onProgress,
        );
      }
      
      onProgress?.call(ProcessingProgress(
        progress: 0.9,
        message: 'Finalizing results...',
        data: {'layout_boxes': layoutBoxes.length},
      ));
      
      // Step 3: Post-process results
      final finalLayoutBoxes = _postProcessLayoutBoxes(layoutBoxes);
      
      stopwatch.stop();
      
      onProgress?.call(ProcessingProgress(
        progress: 1.0,
        message: 'Processing completed successfully',
        data: {'final_boxes': finalLayoutBoxes.length},
      ));
      
      final performanceData = _buildPerformanceData(stopwatch, startTime, finalLayoutBoxes.length);
      
      _logger.performance('AsyncProcessor', 'total_processing', stopwatch.elapsedMilliseconds, {
        'html_length': html.length,
        'final_boxes': finalLayoutBoxes.length,
        'used_chunking': shouldUseChunking,
      });
      
      return ProcessingResult(
        layoutBoxes: finalLayoutBoxes,
        success: true,
        performanceData: performanceData,
      );
      
    } catch (e, stackTrace) {
      stopwatch.stop();
      
      _logger.error('AsyncProcessor', 'Processing failed: $e', {
        'elapsed_ms': stopwatch.elapsedMilliseconds,
        'stack_trace': stackTrace.toString(),
      });
      
      onProgress?.call(ProcessingProgress(
        progress: 1.0,
        message: 'Processing failed: $e',
      ));
      
      return ProcessingResult(
        layoutBoxes: [],
        success: false,
        error: e.toString(),
        performanceData: _buildPerformanceData(stopwatch, startTime, 0),
      );
    }
  }
  
  /// Validates and preprocesses HTML content
  static Future<HtmlValidationResult> _validateAndPreprocessHtml(String html) async {
    try {
      _logger.debug('AsyncProcessor', 'Validating HTML', {'length': html.length});
      
      if (html.isEmpty) {
        return HtmlValidationResult(
          isValid: false,
          error: 'HTML content is empty',
          processedHtml: '',
        );
      }
      
      if (html.length > maxHtmlSizeOverall) {
        return HtmlValidationResult(
          isValid: false,
          error: 'HTML content too large (${html.length} chars, max: $maxHtmlSizeOverall)',
          processedHtml: '',
        );
      }
      
      // Basic HTML preprocessing
      String processedHtml = html;
      
      // Remove potentially problematic content
      processedHtml = _sanitizeHtml(processedHtml);
      
      // Validate basic HTML structure
      if (!_hasBasicHtmlStructure(processedHtml)) {
        _logger.warning('AsyncProcessor', 'HTML lacks basic structure, wrapping in body tag');
        processedHtml = '<html><body>$processedHtml</body></html>';
      }
      
      return HtmlValidationResult(
        isValid: true,
        processedHtml: processedHtml,
      );
      
    } catch (e) {
      _logger.error('AsyncProcessor', 'HTML validation error: $e');
      return HtmlValidationResult(
        isValid: false,
        error: 'HTML validation failed: $e',
        processedHtml: '',
      );
    }
  }
  
  /// Processes HTML using chunking strategy
  static Future<List<LayoutBox>> _processWithChunking(
    String html,
    Function(ProcessingProgress)? onProgress,
    int? customChunkSize,
  ) async {
    try {
      _logger.info('AsyncProcessor', 'Using chunking strategy', {
        'html_length': html.length,
        'custom_chunk_size': customChunkSize,
      });
      
      // Step 1: Create chunks
      onProgress?.call(ProcessingProgress(
        progress: 0.2,
        message: 'Creating HTML chunks...',
      ));
      
      final chunks = HtmlChunker.chunkHtml(html, customChunkSize: customChunkSize);
      
      _logger.debug('AsyncProcessor', 'Created ${chunks.length} chunks');
      
      // Step 2: Process chunks
      onProgress?.call(ProcessingProgress(
        progress: 0.3,
        message: 'Processing ${chunks.length} chunks...',
        data: {'total_chunks': chunks.length},
      ));
      
      final chunkResults = await HtmlChunker.processChunks(
        chunks,
        _processChunkSafely,
        onProgress: (progress) {
          onProgress?.call(ProcessingProgress(
            progress: 0.3 + (progress * 0.5), // 30% to 80%
            message: 'Processing chunks... ${(progress * 100).toInt()}%',
          ));
        },
        onStatusUpdate: (message) {
          onProgress?.call(ProcessingProgress(
            progress: 0.3,
            message: message,
          ));
        },
      );
      
      // Step 3: Combine results
      onProgress?.call(ProcessingProgress(
        progress: 0.8,
        message: 'Combining chunk results...',
        data: {'processed_chunks': chunkResults.length},
      ));
      
      final combinedBoxes = HtmlChunker.combineResults<LayoutBox>(chunkResults);
      
      _logger.info('AsyncProcessor', 'Chunking completed', {
        'total_chunks': chunks.length,
        'successful_chunks': chunkResults.where((r) => r.success).length,
        'combined_boxes': combinedBoxes.length,
      });
      
      return combinedBoxes;
      
    } catch (e) {
      _logger.error('AsyncProcessor', 'Chunking processing error: $e');
      rethrow;
    }
  }
  
  /// Processes HTML in a single pass
  static Future<List<LayoutBox>> _processSinglePass(
    String html,
    Function(ProcessingProgress)? onProgress,
  ) async {
    try {
      _logger.info('AsyncProcessor', 'Using single-pass strategy', {
        'html_length': html.length,
      });
      
      onProgress?.call(ProcessingProgress(
        progress: 0.3,
        message: 'Processing HTML with Rust engine...',
      ));
      
      // Use a timeout for single-pass processing
      final layoutBoxes = await Future.any([
        _processHtmlWithEngine(html),
        Future.delayed(const Duration(seconds: 10), () => <LayoutBox>[]),
      ]);
      
      _logger.info('AsyncProcessor', 'Single-pass completed', {
        'layout_boxes': layoutBoxes.length,
      });
      
      return layoutBoxes;
      
    } catch (e) {
      _logger.error('AsyncProcessor', 'Single-pass processing error: $e');
      rethrow;
    }
  }
  
  /// Safely processes a single chunk
  static Future<List<LayoutBox>> _processChunkSafely(HtmlChunk chunk) async {
    try {
      _logger.debug('AsyncProcessor', 'Processing chunk ${chunk.chunkIndex}', {
        'chunk_size': chunk.content.length,
        'is_first': chunk.isFirst,
        'is_last': chunk.isLast,
      });
      
      // For now, use the demo layout system to avoid FFI crashes
      // TODO: Re-enable full Rust processing once FFI issues are resolved
      return await _processHtmlWithEngine(chunk.content);
      
    } catch (e) {
      _logger.error('AsyncProcessor', 'Chunk processing error: $e', {
        'chunk_index': chunk.chunkIndex,
      });
      
      // Return empty list on error rather than crashing
      return <LayoutBox>[];
    }
  }
  
  /// Processes HTML using the Rust engine
  static Future<List<LayoutBox>> _processHtmlWithEngine(String html) async {
    return await _logger.measurePerformance('AsyncProcessor', 'rust_engine_call', () async {
      try {
        // Use the enhanced EngineBridge with safety measures
        final layoutBoxes = await EngineBridge.parseHtml(html);
        return layoutBoxes;
      } catch (e) {
        _logger.error('AsyncProcessor', 'Rust engine error: $e');
        return <LayoutBox>[];
      }
    });
  }
  
  /// Post-processes layout boxes for final output
  static List<LayoutBox> _postProcessLayoutBoxes(List<LayoutBox> layoutBoxes) {
    try {
      _logger.debug('AsyncProcessor', 'Post-processing layout boxes', {
        'input_count': layoutBoxes.length,
      });
      
      // Remove duplicate or invalid boxes
      final validBoxes = layoutBoxes.where(_isValidLayoutBox).toList();
      
      // Limit the number of boxes to prevent UI performance issues
      final limitedBoxes = validBoxes.take(maxLayoutBoxes).toList();
      
      if (limitedBoxes.length < validBoxes.length) {
        _logger.warning('AsyncProcessor', 'Layout boxes limited', {
          'original_count': validBoxes.length,
          'limited_count': limitedBoxes.length,
          'max_boxes': maxLayoutBoxes,
        });
      }
      
      _logger.debug('AsyncProcessor', 'Post-processing completed', {
        'output_count': limitedBoxes.length,
      });
      
      return limitedBoxes;
      
    } catch (e) {
      _logger.error('AsyncProcessor', 'Post-processing error: $e');
      return layoutBoxes.take(maxLayoutBoxes).toList();
    }
  }
  
  /// Validates a layout box
  static bool _isValidLayoutBox(LayoutBox box) {
    try {
      // Check for reasonable dimensions
      if (box.width < 0 || box.height < 0) return false;
      if (box.width > 10000 || box.height > 10000) return false;
      
      // Check for valid positions
      if (box.x.isNaN || box.y.isNaN) return false;
      if (box.x < -1000 || box.y < -1000) return false;
      
      // Check for valid font size
      if (box.fontSize <= 0 || box.fontSize > 100) return false;
      
      return true;
    } catch (e) {
      _logger.warning('AsyncProcessor', 'Layout box validation error: $e');
      return false;
    }
  }
  
  /// Sanitizes HTML content
  static String _sanitizeHtml(String html) {
    try {
      // Remove potentially problematic scripts and styles
      String sanitized = html;
      
      // Remove script tags (simple approach)
      sanitized = sanitized.replaceAll(RegExp(r'<script[^>]*>.*?</script>', 
          caseSensitive: false, dotAll: true), '');
      
      // Remove problematic style tags that might be too complex
      sanitized = sanitized.replaceAll(RegExp(r'<style[^>]*>.*?</style>', 
          caseSensitive: false, dotAll: true), '');
      
      // Remove comments
      sanitized = sanitized.replaceAll(RegExp(r'<!--.*?-->', dotAll: true), '');
      
      return sanitized;
    } catch (e) {
      _logger.warning('AsyncProcessor', 'HTML sanitization error: $e');
      return html; // Return original on error
    }
  }
  
  /// Checks if HTML has basic structure
  static bool _hasBasicHtmlStructure(String html) {
    final lowerHtml = html.toLowerCase();
    return lowerHtml.contains('<html') || lowerHtml.contains('<body') || lowerHtml.contains('<div');
  }
  
  /// Builds performance data map
  static Map<String, dynamic> _buildPerformanceData(
    Stopwatch stopwatch,
    DateTime startTime,
    int layoutBoxCount,
  ) {
    return {
      'total_time_ms': stopwatch.elapsedMilliseconds,
      'start_time': startTime.toIso8601String(),
      'end_time': DateTime.now().toIso8601String(),
      'layout_box_count': layoutBoxCount,
      'processing_rate': layoutBoxCount > 0 
          ? (layoutBoxCount / (stopwatch.elapsedMilliseconds / 1000.0)).round()
          : 0,
    };
  }
}

class HtmlValidationResult {
  final bool isValid;
  final String? error;
  final String processedHtml;
  
  HtmlValidationResult({
    required this.isValid,
    this.error,
    required this.processedHtml,
  });
}
