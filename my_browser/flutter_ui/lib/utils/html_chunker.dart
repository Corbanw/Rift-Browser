import 'dart:async';
import 'dart:math';
import 'logger.dart';

class HtmlChunk {
  final String content;
  final int chunkIndex;
  final int totalChunks;
  final bool isFirst;
  final bool isLast;
  
  HtmlChunk({
    required this.content,
    required this.chunkIndex,
    required this.totalChunks,
    required this.isFirst,
    required this.isLast,
  });
}

class ChunkProcessingResult {
  final List<dynamic> processedData;
  final int chunkIndex;
  final bool success;
  final String? error;
  
  ChunkProcessingResult({
    required this.processedData,
    required this.chunkIndex,
    required this.success,
    this.error,
  });
}

class HtmlChunker {
  static final Logger _logger = Logger();
  
  // Chunking configuration
  static const int defaultChunkSize = 50000; // 50KB chunks
  static const int maxChunkSize = 100000; // 100KB max
  static const int minChunkSize = 10000; // 10KB min
  static const int overlapSize = 1000; // Overlap between chunks to preserve context
  static const int maxChunks = 20; // Maximum number of chunks to process
  
  // Processing timeouts
  static const Duration chunkProcessingTimeout = Duration(seconds: 2);
  static const Duration totalProcessingTimeout = Duration(seconds: 10);
  
  /// Splits HTML content into manageable chunks
  static List<HtmlChunk> chunkHtml(String html, {int? customChunkSize}) {
    final stopwatch = Stopwatch()..start();
    
    try {
      _logger.debug('HtmlChunker', 'Starting HTML chunking', {
        'html_length': html.length,
        'custom_chunk_size': customChunkSize,
      });
      
      final chunkSize = _calculateOptimalChunkSize(html.length, customChunkSize);
      _logger.debug('HtmlChunker', 'Using chunk size: $chunkSize');
      
      if (html.length <= chunkSize) {
        _logger.debug('HtmlChunker', 'HTML fits in single chunk');
        return [
          HtmlChunk(
            content: html,
            chunkIndex: 0,
            totalChunks: 1,
            isFirst: true,
            isLast: true,
          )
        ];
      }
      
      final chunks = _splitIntoChunks(html, chunkSize);
      
      stopwatch.stop();
      _logger.performance('HtmlChunker', 'chunking', stopwatch.elapsedMilliseconds, {
        'total_chunks': chunks.length,
        'original_size': html.length,
        'chunk_size': chunkSize,
      });
      
      return chunks;
    } catch (e) {
      stopwatch.stop();
      _logger.error('HtmlChunker', 'Error during chunking: $e', {
        'html_length': html.length,
        'elapsed_ms': stopwatch.elapsedMilliseconds,
      });
      
      // Return the original content as a single chunk on error
      return [
        HtmlChunk(
          content: html,
          chunkIndex: 0,
          totalChunks: 1,
          isFirst: true,
          isLast: true,
        )
      ];
    }
  }
  
  /// Processes HTML chunks asynchronously with progress callbacks
  static Future<List<ChunkProcessingResult>> processChunks(
    List<HtmlChunk> chunks,
    Future<dynamic> Function(HtmlChunk chunk) processor,
    {
      Function(double progress)? onProgress,
      Function(String message)? onStatusUpdate,
    }
  ) async {
    final stopwatch = Stopwatch()..start();
    final results = <ChunkProcessingResult>[];
    
    try {
      _logger.info('HtmlChunker', 'Starting chunk processing', {
        'total_chunks': chunks.length,
      });
      
      onStatusUpdate?.call('Processing ${chunks.length} chunks...');
      
      for (int i = 0; i < chunks.length; i++) {
        final chunk = chunks[i];
        
        try {
          _logger.debug('HtmlChunker', 'Processing chunk ${i + 1}/${chunks.length}');
          onStatusUpdate?.call('Processing chunk ${i + 1} of ${chunks.length}...');
          
          // Process chunk with timeout
          final chunkResult = await _processChunkWithTimeout(chunk, processor);
          results.add(chunkResult);
          
          // Update progress
          final progress = (i + 1) / chunks.length;
          onProgress?.call(progress);
          
        } catch (e) {
          _logger.error('HtmlChunker', 'Error processing chunk $i: $e');
          results.add(ChunkProcessingResult(
            processedData: [],
            chunkIndex: i,
            success: false,
            error: e.toString(),
          ));
        }
      }
      
      stopwatch.stop();
      _logger.performance('HtmlChunker', 'chunk_processing', stopwatch.elapsedMilliseconds, {
        'total_chunks': chunks.length,
        'successful_chunks': results.where((r) => r.success).length,
        'failed_chunks': results.where((r) => !r.success).length,
      });
      
      onStatusUpdate?.call('Chunk processing completed');
      return results;
      
    } catch (e) {
      stopwatch.stop();
      _logger.error('HtmlChunker', 'Error during chunk processing: $e', {
        'elapsed_ms': stopwatch.elapsedMilliseconds,
      });
      
      onStatusUpdate?.call('Chunk processing failed: $e');
      return results;
    }
  }
  
  /// Combines results from multiple chunks into a single result
  static List<T> combineResults<T>(List<ChunkProcessingResult> results) {
    final stopwatch = Stopwatch()..start();
    
    try {
      _logger.debug('HtmlChunker', 'Combining chunk results', {
        'total_results': results.length,
      });
      
      final combined = <T>[];
      int successfulChunks = 0;
      
      for (final result in results) {
        if (result.success && result.processedData.isNotEmpty) {
          for (final item in result.processedData) {
            if (item is T) {
              combined.add(item);
            }
          }
          successfulChunks++;
        }
      }
      
      stopwatch.stop();
      _logger.performance('HtmlChunker', 'result_combination', stopwatch.elapsedMilliseconds, {
        'successful_chunks': successfulChunks,
        'combined_items': combined.length,
      });
      
      return combined;
    } catch (e) {
      stopwatch.stop();
      _logger.error('HtmlChunker', 'Error combining results: $e', {
        'elapsed_ms': stopwatch.elapsedMilliseconds,
      });
      return [];
    }
  }
  
  /// Calculates optimal chunk size based on content length
  static int _calculateOptimalChunkSize(int contentLength, int? customSize) {
    if (customSize != null) {
      return customSize.clamp(minChunkSize, maxChunkSize);
    }
    
    // Dynamic chunk size based on content length
    if (contentLength <= 50000) {
      return contentLength; // Single chunk for small content
    } else if (contentLength <= 200000) {
      return 50000; // 50KB chunks for medium content
    } else if (contentLength <= 500000) {
      return 75000; // 75KB chunks for large content
    } else {
      return maxChunkSize; // 100KB chunks for very large content
    }
  }
  
  /// Splits HTML into chunks while preserving tag boundaries
  static List<HtmlChunk> _splitIntoChunks(String html, int chunkSize) {
    final chunks = <HtmlChunk>[];
    int currentPosition = 0;
    int chunkIndex = 0;
    
    while (currentPosition < html.length) {
      final remainingLength = html.length - currentPosition;
      final targetSize = min(chunkSize, remainingLength);
      
      // Find a good break point (end of tag or whitespace)
      int breakPoint = _findBreakPoint(html, currentPosition, targetSize);
      
      // Extract chunk content
      final chunkContent = html.substring(currentPosition, breakPoint);
      
      // Add overlap from previous chunk if not the first chunk
      String finalChunkContent = chunkContent;
      if (chunkIndex > 0 && currentPosition >= overlapSize) {
        final overlapStart = max(0, currentPosition - overlapSize);
        final overlap = html.substring(overlapStart, currentPosition);
        finalChunkContent = overlap + chunkContent;
      }
      
      chunks.add(HtmlChunk(
        content: finalChunkContent,
        chunkIndex: chunkIndex,
        totalChunks: -1, // Will be updated later
        isFirst: chunkIndex == 0,
        isLast: breakPoint >= html.length,
      ));
      
      currentPosition = breakPoint;
      chunkIndex++;
      
      // Safety check to prevent infinite loops
      if (chunkIndex >= maxChunks) {
        _logger.warning('HtmlChunker', 'Reached maximum chunk limit', {
          'max_chunks': maxChunks,
          'processed_length': currentPosition,
          'total_length': html.length,
        });
        break;
      }
    }
    
    // Update total chunks count
    final totalChunks = chunks.length;
    for (int i = 0; i < chunks.length; i++) {
      chunks[i] = HtmlChunk(
        content: chunks[i].content,
        chunkIndex: chunks[i].chunkIndex,
        totalChunks: totalChunks,
        isFirst: chunks[i].isFirst,
        isLast: i == chunks.length - 1,
      );
    }
    
    return chunks;
  }
  
  /// Finds a good break point for chunking (end of tag or whitespace)
  static int _findBreakPoint(String html, int start, int targetSize) {
    final maxPosition = min(start + targetSize, html.length);
    
    // If we're at the end, return the end
    if (maxPosition >= html.length) {
      return html.length;
    }
    
    // Look for a good break point within the last 10% of the chunk
    final searchStart = max(start + (targetSize * 0.9).round(), start + 1);
    
    // Try to find end of a tag
    for (int i = maxPosition - 1; i >= searchStart; i--) {
      if (html[i] == '>') {
        return i + 1;
      }
    }
    
    // Try to find whitespace
    for (int i = maxPosition - 1; i >= searchStart; i--) {
      if (html[i].trim().isEmpty) {
        return i + 1;
      }
    }
    
    // If no good break point found, just use the target size
    return maxPosition;
  }
  
  /// Processes a single chunk with timeout protection
  static Future<ChunkProcessingResult> _processChunkWithTimeout(
    HtmlChunk chunk,
    Future<dynamic> Function(HtmlChunk chunk) processor,
  ) async {
    try {
      final result = await processor(chunk).timeout(chunkProcessingTimeout);
      
      return ChunkProcessingResult(
        processedData: result is List ? result : [result],
        chunkIndex: chunk.chunkIndex,
        success: true,
      );
    } on TimeoutException {
      _logger.warning('HtmlChunker', 'Chunk processing timeout', {
        'chunk_index': chunk.chunkIndex,
        'chunk_size': chunk.content.length,
      });
      
      return ChunkProcessingResult(
        processedData: [],
        chunkIndex: chunk.chunkIndex,
        success: false,
        error: 'Processing timeout',
      );
    } catch (e) {
      _logger.error('HtmlChunker', 'Chunk processing error: $e', {
        'chunk_index': chunk.chunkIndex,
      });
      
      return ChunkProcessingResult(
        processedData: [],
        chunkIndex: chunk.chunkIndex,
        success: false,
        error: e.toString(),
      );
    }
  }
}
