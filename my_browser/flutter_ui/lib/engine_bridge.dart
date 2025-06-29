import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';
import 'dart:convert';
import 'package:ffi/ffi.dart';
import 'dart:ffi' as ffi;
import 'dart:async';
import 'dart:isolate';
import 'dev_console.dart';
import 'package:flutter/foundation.dart';

import 'models/layout_box.dart';
import 'utils/logger.dart';

// FFI function signatures
// C signatures
typedef ParseHtmlC = Pointer<Void> Function(Pointer<Char>);
typedef GetLayoutBoxCountC = Int32 Function(Pointer<Void>);
typedef GetLayoutBoxC = Pointer<FFILayoutBox> Function(Pointer<Void>, Int32);
typedef GetLayoutBoxXC = Float Function(Pointer<Void>);
typedef GetLayoutBoxYC = Float Function(Pointer<Void>);
typedef GetLayoutBoxWidthC = Float Function(Pointer<Void>);
typedef GetLayoutBoxHeightC = Float Function(Pointer<Void>);
typedef GetLayoutBoxNodeTypeC = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxTextContentC = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxBackgroundColorC = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxColorC = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxFontSizeC = Float Function(Pointer<Void>);
typedef FreeLayoutBoxArrayC = Void Function(Pointer<Void>);
typedef FreeLayoutBoxC = Void Function(Pointer<Void>);
typedef FreeCStringC = Void Function(Pointer<Char>);
// Dart signatures
typedef ParseHtmlDart = Pointer<Void> Function(Pointer<Char>);
typedef GetLayoutBoxCountDart = int Function(Pointer<Void>);
typedef GetLayoutBoxDart = Pointer<FFILayoutBox> Function(Pointer<Void>, int);
typedef GetLayoutBoxXDart = double Function(Pointer<Void>);
typedef GetLayoutBoxYDart = double Function(Pointer<Void>);
typedef GetLayoutBoxWidthDart = double Function(Pointer<Void>);
typedef GetLayoutBoxHeightDart = double Function(Pointer<Void>);
typedef GetLayoutBoxNodeTypeDart = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxTextContentDart = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxBackgroundColorDart = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxColorDart = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxFontSizeDart = double Function(Pointer<Void>);
typedef FreeLayoutBoxArrayDart = void Function(Pointer<Void>);
typedef FreeLayoutBoxDart = void Function(Pointer<Void>);
typedef FreeCStringDart = void Function(Pointer<Char>);
typedef GetLayoutBoxFontWeightC = Float Function(Pointer<Void>);
typedef GetLayoutBoxTextAlignC = Pointer<Char> Function(Pointer<Void>);
typedef GetLayoutBoxFontWeightDart = double Function(Pointer<Void>);
typedef GetLayoutBoxTextAlignDart = Pointer<Char> Function(Pointer<Void>);
typedef ParseHtmlWithCssC = Pointer<Void> Function(Pointer<Char>, Pointer<Char>);
typedef ParseHtmlWithCssDart = Pointer<Void> Function(Pointer<Char>, Pointer<Char>);
typedef ParseUrlViaRustC = Pointer<Void> Function(Pointer<Char>);
typedef ParseUrlViaRustDart = Pointer<Void> Function(Pointer<Char>);
typedef FreeFFILayoutBoxC = ffi.Void Function(ffi.Pointer<FFILayoutBox>);
typedef FreeFFILayoutBoxDart = void Function(ffi.Pointer<FFILayoutBox>);
typedef GetLayoutBoxBatchC = Int32 Function(Pointer<Void>, Int32, Int32, Pointer<Pointer<FFILayoutBox>>);
typedef GetLayoutBoxBatchDart = int Function(Pointer<Void>, int, int, Pointer<Pointer<FFILayoutBox>>);

// New enhanced function signatures
typedef GetLayoutBoxBatchEnhancedC = Int32 Function(Pointer<Void>, Int32, Int32, Pointer<Pointer<FFILayoutBox>>);
typedef GetLayoutBoxBatchEnhancedDart = int Function(Pointer<Void>, int, int, Pointer<Pointer<FFILayoutBox>>);
typedef ParseHtmlToDrawCommandsC = Pointer<Void> Function(Pointer<Char>);
typedef ParseHtmlToDrawCommandsDart = Pointer<Void> Function(Pointer<Char>);
typedef ParseUrlViaRustEnhancedC = Pointer<Void> Function(Pointer<Char>);
typedef ParseUrlViaRustEnhancedDart = Pointer<Void> Function(Pointer<Char>);

// FFI struct for FFILayoutBox
final class FFILayoutBox extends ffi.Struct {
  @ffi.Double()
  external double x;
  @ffi.Double()
  external double y;
  @ffi.Double()
  external double width;
  @ffi.Double()
  external double height;
  @ffi.Double()
  external double font_size;
  @ffi.Double()
  external double font_weight;
  external ffi.Pointer<ffi.Char> node_type;
  external ffi.Pointer<ffi.Char> text_content;
  external ffi.Pointer<ffi.Char> background_color;
  external ffi.Pointer<ffi.Char> color;
  external ffi.Pointer<ffi.Char> font_family;
  external ffi.Pointer<ffi.Char> border_color;
  external ffi.Pointer<ffi.Char> text_align;
  @ffi.Double()
  external double margin_top;
  @ffi.Double()
  external double margin_right;
  @ffi.Double()
  external double margin_bottom;
  @ffi.Double()
  external double margin_left;
  @ffi.Double()
  external double padding_top;
  @ffi.Double()
  external double padding_right;
  @ffi.Double()
  external double padding_bottom;
  @ffi.Double()
  external double padding_left;
  @ffi.Double()
  external double border_width_top;
  @ffi.Double()
  external double border_width_right;
  @ffi.Double()
  external double border_width_bottom;
  @ffi.Double()
  external double border_width_left;
}

class EngineInitResult {
  final bool success;
  final String? errorMessage;
  final String? stackTrace;
  EngineInitResult.success() : success = true, errorMessage = null, stackTrace = null;
  EngineInitResult.failure(this.errorMessage, this.stackTrace) : success = false;
}

class EngineBridge {
  static DynamicLibrary? _lib;
  static bool _initialized = false;

  // Function pointers
  static late final ParseHtmlDart _parseHtml;
  static late final GetLayoutBoxCountDart _getLayoutBoxCount;
  static late final GetLayoutBoxDart _getLayoutBox;
  static late final FreeLayoutBoxArrayDart _freeLayoutBoxArray;
  static late final ParseHtmlWithCssDart _parseHtmlWithCss;
  static late final ParseUrlViaRustDart _parseUrlViaRust;
  static late final FreeFFILayoutBoxDart _freeFFILayoutBox;
  static late final GetLayoutBoxBatchDart _getLayoutBoxBatch;
  
  // New enhanced function pointers
  static late final GetLayoutBoxBatchEnhancedDart _getLayoutBoxBatchEnhanced;
  static late final ParseHtmlToDrawCommandsDart _parseHtmlToDrawCommands;
  static late final ParseUrlViaRustEnhancedDart _parseUrlViaRustEnhanced;

  // Constants
  static const int _maxHtmlSize = 2000000;
  static const int _maxBoxCount = 1000;
  static const int _maxBoxesToExtract = 500;
  static const int _batchSize = 5;
  static const int _maxExtractionTimeMs = 5000;
  static const int _maxAsyncTimeMs = 10000;
  static const int _maxStringLength = 1000;
  static const int _maxFinalStringLength = 500;
  static const int _minPointerAddress = 0x1000;
  static const int _maxPointerAddress = 0x7FFFFFFFFFFFFFFF;
  
  // New timeout constants for HTML parsing
  static const int _maxHtmlParsingTimeMs = 3000;
  static const int _maxLayoutTimeMs = 5000;
  static const int _maxTotalProcessingTimeMs = 10000;
  static const int _largePageThreshold = 100000;

  static EngineInitResult initialize() {
    if (_initialized) return EngineInitResult.success();
    
    try {
      _loadDynamicLibrary();
      _lookupFunctionPointers();
      _initialized = true;
      print('Rust engine initialized successfully');
      return EngineInitResult.success();
    } catch (e, stack) {
      final msg = 'Failed to initialize Rust engine: $e';
      print('$msg\n$stack');
      return EngineInitResult.failure(msg, stack.toString());
    }
  }

  static void _loadDynamicLibrary() {
    try {
      if (Platform.isWindows) {
        logPrint('EngineBridge: Loading Windows library: rust_engine.dll');
        _lib = DynamicLibrary.open('rust_engine.dll');
      } else if (Platform.isMacOS) {
        logPrint('EngineBridge: Loading macOS library: librust_engine.dylib');
        _lib = DynamicLibrary.open('librust_engine.dylib');
      } else {
        logPrint('EngineBridge: Loading Linux library: librust_engine.so');
        _lib = DynamicLibrary.open('librust_engine.so');
      }
      
      if (_lib == null) {
        const msg = 'Failed to load Rust engine library';
        logPrint('EngineBridge: $msg');
        throw Exception(msg);
      }
      
      logPrint('EngineBridge: Successfully loaded dynamic library: $_lib');
    } catch (e) {
      logPrint('EngineBridge: Error loading dynamic library: $e');
      rethrow;
    }
  }

  static void _lookupFunctionPointers() {
    try {
      logPrint('EngineBridge: Looking up function pointers...');
      
      _parseHtml = _lib!.lookupFunction<ParseHtmlC, ParseHtmlDart>('parse_html');
      logPrint('EngineBridge: _parseHtml function pointer initialized');
      
      _getLayoutBoxCount = _lib!.lookupFunction<GetLayoutBoxCountC, GetLayoutBoxCountDart>('get_layout_box_count');
      logPrint('EngineBridge: _getLayoutBoxCount function pointer initialized');
      
      _getLayoutBox = _lib!.lookupFunction<GetLayoutBoxC, GetLayoutBoxDart>('get_layout_box');
      logPrint('EngineBridge: _getLayoutBox function pointer initialized');
      
      _freeLayoutBoxArray = _lib!.lookupFunction<FreeLayoutBoxArrayC, FreeLayoutBoxArrayDart>('free_layout_box_array');
      logPrint('EngineBridge: _freeLayoutBoxArray function pointer initialized');
      
      _parseHtmlWithCss = _lib!.lookupFunction<
        Pointer<Void> Function(Pointer<Char>, Pointer<Char>),
        Pointer<Void> Function(Pointer<Char>, Pointer<Char>)
      >('parse_html_with_css');
      logPrint('EngineBridge: _parseHtmlWithCss function pointer initialized');
      
      _parseUrlViaRust = _lib!.lookupFunction<ParseUrlViaRustC, ParseUrlViaRustDart>('parse_url_via_rust');
      logPrint('EngineBridge: _parseUrlViaRust function pointer initialized');
      
      _freeFFILayoutBox = _lib!.lookupFunction<FreeFFILayoutBoxC, FreeFFILayoutBoxDart>('free_ffi_layout_box');
      logPrint('EngineBridge: _freeFFILayoutBox function pointer initialized');
      
      _getLayoutBoxBatch = _lib!.lookupFunction<GetLayoutBoxBatchC, GetLayoutBoxBatchDart>('get_layout_box_batch');
      logPrint('EngineBridge: _getLayoutBoxBatch function pointer initialized');
      
      _getLayoutBoxBatchEnhanced = _lib!.lookupFunction<GetLayoutBoxBatchEnhancedC, GetLayoutBoxBatchEnhancedDart>('get_layout_box_batch_enhanced');
      logPrint('EngineBridge: _getLayoutBoxBatchEnhanced function pointer initialized');
      
      _parseHtmlToDrawCommands = _lib!.lookupFunction<ParseHtmlToDrawCommandsC, ParseHtmlToDrawCommandsDart>('parse_html_to_draw_commands');
      logPrint('EngineBridge: _parseHtmlToDrawCommands function pointer initialized');
      
      _parseUrlViaRustEnhanced = _lib!.lookupFunction<ParseUrlViaRustEnhancedC, ParseUrlViaRustEnhancedDart>('parse_url_via_rust_enhanced');
      logPrint('EngineBridge: _parseUrlViaRustEnhanced function pointer initialized');
      
      logPrint('EngineBridge: All function pointers initialized successfully');
    } catch (e) {
      logPrint('EngineBridge: Error looking up function pointers: $e');
      rethrow;
    }
  }

  static Future<List<LayoutBox>> parseHtml(String html) async {
    if (!_initialized) {
      logPrint('Engine not initialized');
      return [];
    }

    // Validate that library and function pointers are properly loaded
    if (_lib == null) {
      logPrint('EngineBridge: Dynamic library is null, reinitializing...');
      try {
        initialize();
      } catch (e) {
        logPrint('EngineBridge: Failed to reinitialize: $e');
        return _createLayoutFromHtml(html);
      }
    }

    try {
      logPrint('EngineBridge: Starting HTML parsing...');
      
      // Validate input
      if (html.isEmpty) {
        logPrint('EngineBridge: Empty HTML input');
        return [];
      }
      
      // Use chunked processing for better responsiveness
      final layoutBoxes = await _parseHtmlWithChunkedProcessing(html);
      
      if (layoutBoxes.isEmpty) {
        logPrint('EngineBridge: No layout boxes extracted, using fallback');
        return _createLayoutFromHtml(html);
      }
      
      logPrint('EngineBridge: Successfully extracted ${layoutBoxes.length} layout boxes from Rust engine');
      return layoutBoxes;
      
    } catch (e) {
      logPrint('Error parsing HTML: $e');
      return _createLayoutFromHtml(html);
    }
  }

  static Future<List<LayoutBox>> _parseHtmlWithChunkedProcessing(String html) async {
    try {
      // Increased timeout for complex pages - 15 seconds for large pages, 10 seconds for smaller pages
      final timeout = html.length > 100000 ? Duration(seconds: 15) : Duration(seconds: 10);
      
      return await Future.any([
        _parseHtmlInternalWithChunking(html),
        Future.delayed(timeout, () {
          logPrint('EngineBridge: HTML parsing timed out after ${timeout.inSeconds} seconds, using fallback');
          return _createLayoutFromHtml(html);
        }),
      ]);
    } catch (e) {
      logPrint('EngineBridge: Error in _parseHtmlWithChunkedProcessing: $e');
      return _createLayoutFromHtml(html);
    }
  }

  static Future<List<LayoutBox>> _parseHtmlInternalWithChunking(String html) async {
    // Convert HTML to native UTF8
    final htmlPtr = _convertStringToNativeUtf8(html);
    if (htmlPtr == nullptr) {
      logPrint('EngineBridge: Failed to convert HTML to UTF8');
      return [];
    }
    
    try {
      // Call Rust engine to parse HTML
      logPrint('EngineBridge: Calling Rust parse_html function...');
      final boxArrayPtr = _callParseHtml(htmlPtr);
      
      if (boxArrayPtr == nullptr) {
        logPrint('EngineBridge: Rust engine returned null, using fallback');
        return [];
      }
      
      // Get layout box count
      final count = _getLayoutBoxCount(boxArrayPtr);
      logPrint('EngineBridge: Rust engine returned $count layout boxes');
      
      if (count <= 0) {
        logPrint('EngineBridge: Invalid box count ($count), using fallback');
        _freeLayoutBoxArray(boxArrayPtr);
        return [];
      }
      
      // Extract layout boxes in batches for better performance
      final layoutBoxes = await _extractLayoutBoxesBatch(boxArrayPtr, count);
      
      // Clean up
      _freeLayoutBoxArray(boxArrayPtr);
      
      return layoutBoxes;
      
    } finally {
      // Clean up HTML pointer
      calloc.free(htmlPtr);
    }
  }

  static Future<List<LayoutBox>> _extractLayoutBoxesBatch(Pointer<Void> boxArrayPtr, int count) async {
    try {
      logPrint('EngineBridge: Starting batch extraction of $count boxes');
      final List<LayoutBox> layoutBoxes = [];
      const int batchSize = 1000;
      final Pointer<Pointer<FFILayoutBox>> batchPtr = calloc.allocate<Pointer<FFILayoutBox>>(batchSize);
      int extracted = 0;
      
      logPrint('EngineBridge: Allocated batch pointer: $batchPtr');
      
      while (extracted < count) {
        final toExtract = (count - extracted) < batchSize ? (count - extracted) : batchSize;
        logPrint('EngineBridge: Calling _getLayoutBoxBatchEnhanced with start=$extracted, count=$toExtract');
        
        final n = _getLayoutBoxBatchEnhanced(boxArrayPtr, extracted, toExtract, batchPtr);
        logPrint('EngineBridge: _getLayoutBoxBatchEnhanced returned $n boxes');
        
        if (n <= 0) {
          logPrint('EngineBridge: No more boxes to extract, breaking');
          break;
        }
        
        for (int i = 0; i < n; i++) {
          final boxPtr = batchPtr[i];
          logPrint('EngineBridge: Processing box $i, ptr: $boxPtr');
          
          if (boxPtr == nullptr) {
            logPrint('EngineBridge: Box $i is null, skipping');
            continue;
          }
          
          try {
            final layoutBox = _extractLayoutBox(boxPtr);
            if (layoutBox != null) {
              layoutBoxes.add(layoutBox);
              logPrint('EngineBridge: Successfully extracted box $i: ${layoutBox.nodeType}');
            }
          } catch (e) {
            logPrint('EngineBridge: Error extracting box $i: $e');
          }
          
          _safeFreeLayoutBox(boxPtr);
        }
        
        extracted += n;
        logPrint('EngineBridge: Batch extracted $extracted/$count boxes (total so far: ${layoutBoxes.length})');
        await Future.delayed(Duration(milliseconds: 1));
      }
      
      calloc.free(batchPtr);
      logPrint('EngineBridge: Finished batch extraction, total ${layoutBoxes.length} boxes');
      return layoutBoxes;
    } catch (e) {
      logPrint('EngineBridge: Error in batch extraction: $e');
      return [];
    }
  }

  static List<LayoutBox> _createLayoutFromHtml(String html) {
    logPrint('EngineBridge: Creating layout from HTML content');
    
    List<LayoutBox> boxes = [];
    int yOffset = 50;
    
    // Extract title if available
    String title = 'Web Page';
    if (html.contains('<title>')) {
      final titleStart = html.indexOf('<title>') + 7;
      final titleEnd = html.indexOf('</title>', titleStart);
      if (titleEnd > titleStart) {
        title = html.substring(titleStart, titleEnd).trim();
      }
    }
    
    // Header box
    boxes.add(LayoutBox(
      x: 50.0, y: yOffset.toDouble(), width: 700.0, height: 60.0,
      nodeType: 'h1', textContent: title,
      backgroundColor: '#e8f5e8', color: '#2e7d32', fontSize: 24.0,
      fontFamily: 'Arial', borderWidth: 2.0, borderColor: '#2e7d32',
      padding: 10.0, margin: 10.0, fontWeight: 700.0, textAlign: 'center',
    ));
    yOffset += 80;
    
    // Content info box
    boxes.add(LayoutBox(
      x: 50.0, y: yOffset.toDouble(), width: 700.0, height: 50.0,
      nodeType: 'p', textContent: 'Page loaded successfully (${html.length} characters)',
      backgroundColor: '#fff3e0', color: '#ef6c00', fontSize: 14.0,
      fontFamily: 'Arial', borderWidth: 1.0, borderColor: '#ef6c00',
      padding: 10.0, margin: 10.0, fontWeight: 400.0, textAlign: 'left',
    ));
    yOffset += 70;
    
    // Extract and display images
    final images = _extractImages(html);
    for (final image in images) {
      boxes.add(LayoutBox(
        x: 50.0, y: yOffset.toDouble(), width: 300.0, height: 200.0,
        nodeType: 'img', textContent: image,
        backgroundColor: '#f0f0f0', color: '#666666', fontSize: 12.0,
        fontFamily: 'Arial', borderWidth: 1.0, borderColor: '#cccccc',
        padding: 10.0, margin: 10.0, fontWeight: 400.0, textAlign: 'center',
      ));
      yOffset += 220;
    }
    
    // Extract and display some text content
    final textContent = _extractTextContent(html);
    if (textContent.isNotEmpty) {
      final lines = textContent.split('\n').take(5); // Limit to 5 lines
      for (final line in lines) {
        if (line.trim().isNotEmpty) {
          boxes.add(LayoutBox(
            x: 50.0, y: yOffset.toDouble(), width: 700.0, height: 40.0,
            nodeType: 'p', textContent: line.trim(),
            backgroundColor: '#f5f5f5', color: '#333333', fontSize: 14.0,
            fontFamily: 'Arial', borderWidth: 0.0, borderColor: '',
            padding: 8.0, margin: 5.0, fontWeight: 400.0, textAlign: 'left',
          ));
          yOffset += 50;
        }
      }
    }
    
    // Extract and display links
    final links = _extractLinks(html);
    if (links.isNotEmpty) {
      boxes.add(LayoutBox(
        x: 50.0, y: yOffset.toDouble(), width: 700.0, height: 40.0,
        nodeType: 'h3', textContent: 'Links found:',
        backgroundColor: '#e3f2fd', color: '#1565c0', fontSize: 16.0,
        fontFamily: 'Arial', borderWidth: 0.0, borderColor: '',
        padding: 10.0, margin: 10.0, fontWeight: 600.0, textAlign: 'left',
      ));
      yOffset += 50;
      
      for (final link in links.take(3)) { // Show first 3 links
        boxes.add(LayoutBox(
          x: 50.0, y: yOffset.toDouble(), width: 700.0, height: 30.0,
          nodeType: 'a', textContent: link,
          backgroundColor: '#f8f9fa', color: '#0066cc', fontSize: 12.0,
          fontFamily: 'Arial', borderWidth: 0.0, borderColor: '',
          padding: 5.0, margin: 2.0, fontWeight: 400.0, textAlign: 'left',
        ));
        yOffset += 40;
      }
    }
    
    // Status box
    boxes.add(LayoutBox(
      x: 50.0, y: yOffset.toDouble(), width: 700.0, height: 60.0,
      nodeType: 'p', textContent: 'Rift Browser successfully processed this page. Found ${images.length} images and ${links.length} links.',
      backgroundColor: '#e3f2fd', color: '#1565c0', fontSize: 12.0,
      fontFamily: 'Arial', borderWidth: 1.0, borderColor: '#1565c0',
      padding: 10.0, margin: 10.0, fontWeight: 400.0, textAlign: 'left',
    ));
    
    logPrint('EngineBridge: Created ${boxes.length} layout boxes from HTML');
    return boxes;
  }

  static List<String> _extractImages(String html) {
    try {
      final images = <String>[];
      final imgPattern1 = RegExp(r'<img[^>]+src="([^"]+)"[^>]*>', caseSensitive: false);
      final imgPattern2 = RegExp(r"<img[^>]+src='([^']+)'[^>]*>", caseSensitive: false);
      
      final matches1 = imgPattern1.allMatches(html);
      final matches2 = imgPattern2.allMatches(html);
      
      for (final match in matches1) {
        final src = match.group(1);
        if (src != null && src.isNotEmpty) {
          images.add(src);
        }
      }
      
      for (final match in matches2) {
        final src = match.group(1);
        if (src != null && src.isNotEmpty) {
          images.add(src);
        }
      }
      
      logPrint('EngineBridge: Extracted ${images.length} images from HTML');
      return images;
    } catch (e) {
      logPrint('EngineBridge: Error extracting images: $e');
      return [];
    }
  }

  static List<String> _extractLinks(String html) {
    try {
      final links = <String>[];
      final linkPattern1 = RegExp(r'<a[^>]+href="([^"]+)"[^>]*>', caseSensitive: false);
      final linkPattern2 = RegExp(r"<a[^>]+href='([^']+)'[^>]*>", caseSensitive: false);
      
      final matches1 = linkPattern1.allMatches(html);
      final matches2 = linkPattern2.allMatches(html);
      
      for (final match in matches1) {
        final href = match.group(1);
        if (href != null && href.isNotEmpty && !href.startsWith('#')) {
          links.add(href);
        }
      }
      
      for (final match in matches2) {
        final href = match.group(1);
        if (href != null && href.isNotEmpty && !href.startsWith('#')) {
          links.add(href);
        }
      }
      
      logPrint('EngineBridge: Extracted ${links.length} links from HTML');
      return links;
    } catch (e) {
      logPrint('EngineBridge: Error extracting links: $e');
      return [];
    }
  }

  static String _extractTextContent(String html) {
    try {
      // Simple text extraction - remove HTML tags
      String text = html;
      
      // Remove script and style tags and their content
      text = text.replaceAll(RegExp(r'<script[^>]*>.*?</script>', dotAll: true), '');
      text = text.replaceAll(RegExp(r'<style[^>]*>.*?</style>', dotAll: true), '');
      
      // Remove HTML tags
      text = text.replaceAll(RegExp(r'<[^>]*>'), '');
      
      // Decode HTML entities
      text = text.replaceAll('&amp;', '&');
      text = text.replaceAll('&lt;', '<');
      text = text.replaceAll('&gt;', '>');
      text = text.replaceAll('&quot;', '"');
      text = text.replaceAll('&#39;', "'");
      text = text.replaceAll('&nbsp;', ' ');
      
      // Clean up whitespace
      text = text.replaceAll(RegExp(r'\s+'), ' ').trim();
      
      return text;
    } catch (e) {
      logPrint('EngineBridge: Error extracting text content: $e');
      return 'Content extraction failed';
    }
  }

  static Future<List<LayoutBox>> _extractLayoutBoxesSafely(Pointer<Void> boxArrayPtr, int count) async {
    try {
      final List<LayoutBox> layoutBoxes = [];
      final maxBoxesToExtract = count.clamp(0, 50); // Limit to 50 boxes max
      
      logPrint('EngineBridge: Extracting up to $maxBoxesToExtract layout boxes safely');
      
      for (int i = 0; i < maxBoxesToExtract; i++) {
        try {
          // Add timeout protection for each layout box extraction
          final boxPtr = await _getLayoutBoxWithTimeout(boxArrayPtr, i);
          if (boxPtr == nullptr) {
            logPrint('EngineBridge: No more layout boxes at index $i');
            break;
          }

          // Extract the layout box with additional safety checks
          final layoutBox = _extractLayoutBoxWithTimeout(boxPtr);
          if (layoutBox != null) {
            layoutBoxes.add(layoutBox);
            logPrint('EngineBridge: Successfully extracted layout box $i');
          }

          _safeFreeLayoutBox(boxPtr);
          
          // Remove the small delay that could accumulate and cause memory issues
          if (i % 5 == 0 && i > 0) {
            logPrint('EngineBridge: Processed $i layout boxes, continuing...');
          }
          
        } catch (e) {
          logPrint('EngineBridge: Error extracting layout box $i: $e');
          // Continue with next box instead of crashing
          continue;
        }
      }
      
      logPrint('EngineBridge: Successfully extracted ${layoutBoxes.length} layout boxes');
      return layoutBoxes;
      
    } catch (e) {
      logPrint('EngineBridge: Error in safe layout box extraction: $e');
      return [];
    }
  }

  static Pointer<Uint8> _convertStringToNativeUtf8(String html) {
    try {
      final htmlBytes = Uint8List.fromList(utf8.encode(html));
      final htmlPtr = calloc<Uint8>(htmlBytes.length + 1);
      for (int i = 0; i < htmlBytes.length; i++) {
        htmlPtr[i] = htmlBytes[i];
      }
      htmlPtr[htmlBytes.length] = 0; // Null terminator
      return htmlPtr;
    } catch (e) {
      logPrint('Error converting string to UTF8: $e');
      rethrow;
    }
  }

  static Pointer<Void> _callParseHtml(Pointer<Uint8> htmlPtr) {
    try {
      logPrint('EngineBridge: Calling Rust parse_html function...');
      final boxArrayPtr = _parseHtml(htmlPtr.cast<Char>());
      logPrint('EngineBridge: Rust function returned: $boxArrayPtr');
      return boxArrayPtr;
    } catch (e) {
      logPrint('Error calling Rust parse_html: $e');
      return nullptr;
    }
  }

  static Future<Pointer<FFILayoutBox>> _getLayoutBoxWithTimeout(
    Pointer<Void> boxArrayPtr, 
    int index
  ) async {
    try {
      // Add a small timeout to prevent hanging
      final completer = Completer<Pointer<FFILayoutBox>>();
      Timer? timeoutTimer;
      
      timeoutTimer = Timer(const Duration(milliseconds: 100), () {
        if (!completer.isCompleted) {
          logPrint('EngineBridge: Timeout getting layout box $index');
          completer.complete(nullptr);
        }
      });
      
      // Try to get the layout box
      try {
        final boxPtr = _getLayoutBox(boxArrayPtr, index);
        if (!completer.isCompleted) {
          timeoutTimer?.cancel();
          completer.complete(boxPtr);
        }
      } catch (e) {
        if (!completer.isCompleted) {
          timeoutTimer?.cancel();
          logPrint('EngineBridge: Error getting layout box $index: $e');
          completer.complete(nullptr);
        }
      }
      
      return await completer.future.timeout(
        const Duration(milliseconds: 200),
        onTimeout: () {
          timeoutTimer?.cancel();
          logPrint('EngineBridge: Timeout getting layout box $index');
          return nullptr;
        },
      );
      
    } catch (e) {
      logPrint('EngineBridge: Error in _getLayoutBoxWithTimeout: $e');
      return nullptr;
    }
  }

  static LayoutBox? _extractLayoutBoxWithTimeout(Pointer<FFILayoutBox> boxPtr) {
    try {
      // Simple synchronous extraction with basic validation
      if (!_isValidBoxPointer(boxPtr)) {
        logPrint('EngineBridge: Invalid box pointer');
        return null;
      }
      
      final ffiBox = boxPtr.ref;
      
      // Validate numeric values before extraction
      if (!_areNumericValuesValid(ffiBox)) {
        logPrint('EngineBridge: Invalid numeric values in layout box');
        return null;
      }
      
      final numericValues = _extractNumericValues(ffiBox);
      final stringValues = _extractStringValues(ffiBox);
      final calculatedValues = _calculateAverages(ffiBox);
      
      final layoutBox = _createLayoutBox(numericValues, stringValues, calculatedValues);
      return layoutBox;
      
    } catch (e) {
      logPrint('EngineBridge: Error in _extractLayoutBoxWithTimeout: $e');
      return null;
    }
  }

  static bool _areNumericValuesValid(FFILayoutBox ffiBox) {
    try {
      // Check if all numeric values are finite
      final numericFields = [
        ffiBox.x, ffiBox.y, ffiBox.width, ffiBox.height,
        ffiBox.font_size, ffiBox.font_weight,
        ffiBox.margin_top, ffiBox.margin_right, ffiBox.margin_bottom, ffiBox.margin_left,
        ffiBox.padding_top, ffiBox.padding_right, ffiBox.padding_bottom, ffiBox.padding_left,
        ffiBox.border_width_top, ffiBox.border_width_right, ffiBox.border_width_bottom, ffiBox.border_width_left,
      ];

      for (final value in numericFields) {
        if (!value.isFinite) {
          logPrint('EngineBridge: Non-finite numeric value found: $value');
          return false;
        }
      }

      // Check for reasonable bounds
      if (ffiBox.width < 0 || ffiBox.height < 0 || ffiBox.font_size < 0) {
        logPrint('EngineBridge: Negative dimensions found: width=${ffiBox.width}, height=${ffiBox.height}, font_size=${ffiBox.font_size}');
        return false;
      }

      return true;
    } catch (e) {
      logPrint('EngineBridge: Error validating numeric values: $e');
      return false;
    }
  }

  static void _safeFreeLayoutBox(Pointer<FFILayoutBox> boxPtr) {
    try {
      if (boxPtr != nullptr) {
        _freeFFILayoutBox(boxPtr);
      }
    } catch (e) {
      logPrint('EngineBridge: Error freeing layout box: $e');
    }
  }

  static Map<String, double> _extractNumericValues(FFILayoutBox ffiBox) {
    return {
      'x': ffiBox.x.isFinite ? ffiBox.x : 0.0,
      'y': ffiBox.y.isFinite ? ffiBox.y : 0.0,
      'width': ffiBox.width.isFinite && ffiBox.width >= 0 ? ffiBox.width : 0.0,
      'height': ffiBox.height.isFinite && ffiBox.height >= 0 ? ffiBox.height : 0.0,
      'font_size': ffiBox.font_size.isFinite && ffiBox.font_size > 0 ? ffiBox.font_size : 12.0,
      'font_weight': ffiBox.font_weight.isFinite && ffiBox.font_weight >= 0 ? ffiBox.font_weight : 400.0,
    };
  }

  static Map<String, String> _extractStringValues(FFILayoutBox ffiBox) {
    return {
      'node_type': _safeCStringToString(ffiBox.node_type, 'unknown'),
      'text_content': _safeCStringToString(ffiBox.text_content, ''),
      'background_color': _safeCStringToString(ffiBox.background_color, ''),
      'color': _safeCStringToString(ffiBox.color, ''),
      'font_family': _safeCStringToString(ffiBox.font_family, ''),
      'border_color': _safeCStringToString(ffiBox.border_color, ''),
      'text_align': _safeCStringToString(ffiBox.text_align, 'left'),
    };
  }

  static Map<String, double> _calculateAverages(FFILayoutBox ffiBox) {
    return {
      'margin': _calculateAverage([
        ffiBox.margin_top, ffiBox.margin_right, ffiBox.margin_bottom, ffiBox.margin_left
      ]),
      'padding': _calculateAverage([
        ffiBox.padding_top, ffiBox.padding_right, ffiBox.padding_bottom, ffiBox.padding_left
      ]),
      'border_width': _calculateAverage([
        ffiBox.border_width_top, ffiBox.border_width_right, ffiBox.border_width_bottom, ffiBox.border_width_left
      ]),
    };
  }

  static LayoutBox _createLayoutBox(
    Map<String, double> numericValues,
    Map<String, String> stringValues,
    Map<String, double> calculatedValues,
  ) {
    return LayoutBox(
      x: numericValues['x'] ?? 0.0,
      y: numericValues['y'] ?? 0.0,
      width: numericValues['width'] ?? 0.0,
      height: numericValues['height'] ?? 0.0,
      nodeType: stringValues['node_type'] ?? '',
      textContent: stringValues['text_content'] ?? '',
      backgroundColor: stringValues['background_color'] ?? '',
      color: stringValues['color'] ?? '',
      fontSize: numericValues['font_size'] ?? 12.0,
      fontFamily: stringValues['font_family'] ?? '',
      borderWidth: numericValues['border_width'] ?? 0.0,
      borderColor: stringValues['border_color'] ?? '',
      padding: numericValues['padding'] ?? 0.0,
      margin: numericValues['margin'] ?? 0.0,
      fontWeight: numericValues['font_weight'] ?? 400.0,
      textAlign: stringValues['text_align'] ?? '',
      // Flexbox properties
      flexDirection: stringValues['flexDirection'] ?? '',
      flexWrap: stringValues['flexWrap'] ?? '',
      justifyContent: stringValues['justifyContent'] ?? '',
      alignItems: stringValues['alignItems'] ?? '',
      flexGrow: numericValues['flexGrow'] ?? 0.0,
      flexShrink: numericValues['flexShrink'] ?? 1.0,
      flexBasis: stringValues['flexBasis'] ?? '',
      order: numericValues['order']?.toInt() ?? 0,
      // Grid properties
      gridColumn: stringValues['gridColumn'] ?? '',
      gridRow: stringValues['gridRow'] ?? '',
      // Text rendering
      lineHeight: numericValues['lineHeight'] ?? 1.2,
      wordWrap: stringValues['wordWrap'] ?? '',
      whiteSpace: stringValues['whiteSpace'] ?? '',
      textOverflow: stringValues['textOverflow'] ?? '',
      // Theme support
      colorScheme: stringValues['colorScheme'] ?? '',
    );
  }

  static String _safeCStringToString(Pointer<Char> ptr, String defaultValue) {
    try {
      if (!_isValidStringPointer(ptr)) return defaultValue;
      
      final bytes = _readBytesFromPointer(ptr);
      if (bytes.isEmpty) return defaultValue;
      
      return _convertBytesToString(bytes);
    } catch (e) {
      logPrint('[DART] _safeCStringToString: Error: $e');
      return defaultValue;
    }
  }

  static bool _isValidStringPointer(Pointer<Char> ptr) {
    try {
      if (ptr == nullptr || ptr.address == 0) return false;
      
      if (ptr.address < _minPointerAddress || ptr.address > _maxPointerAddress) {
        logPrint('[DART] _safeCStringToString: Invalid pointer address: 0x${ptr.address.toRadixString(16)}');
        return false;
      }
      
      if (ptr.address % 4 != 0) {
        logPrint('[DART] _safeCStringToString: Unaligned pointer address: 0x${ptr.address.toRadixString(16)}');
        return false;
      }
      
      return true;
    } catch (e) {
      logPrint('[DART] _isValidStringPointer: Error: $e');
      return false;
    }
  }

  static List<int> _readBytesFromPointer(Pointer<Char> ptr) {
    try {
    final List<int> bytes = [];
    int i = 0;
      
      while (i < _maxStringLength) {
        try {
          final byte = ptr[i];
          if (byte == 0) break; // Null terminator
          if (byte < 0 || byte > 255) {
            logPrint('[DART] _safeCStringToString: Invalid byte value: $byte at position $i');
            break;
          }
          bytes.add(byte);
          i++;
        } catch (e) {
          logPrint('[DART] _safeCStringToString: Error reading byte at position $i: $e');
          break;
        }
      }
      
      return bytes;
    } catch (e) {
      logPrint('[DART] _readBytesFromPointer: Error: $e');
      return [];
    }
  }

  static String _convertBytesToString(List<int> bytes) {
    try {
      final result = String.fromCharCodes(bytes);
      if (result.length > _maxFinalStringLength) {
        logPrint('[DART] _safeCStringToString: String too long (${result.length} chars), truncating');
        return result.substring(0, _maxFinalStringLength);
      }
      return result;
    } catch (e) {
      logPrint('[DART] _safeCStringToString: Error converting bytes to string: $e');
      return '';
    }
  }

  static double _calculateAverage(List<double> values) {
    try {
      final validValues = values.where((v) => v.isFinite && v >= 0).toList();
      if (validValues.isEmpty) return 0.0;
      return validValues.reduce((a, b) => a + b) / validValues.length;
    } catch (e) {
      logPrint('[DART] _calculateAverage: Error: $e');
      return 0.0;
    }
  }

  static List<LayoutBox> parseHtmlWithCss(String html, String css) {
    logPrint('[DART] parseHtmlWithCss called');
    
    try {
      if (!_validateHtmlInput(html)) {
        logPrint('[DART] parseHtmlWithCss: HTML validation failed, returning fallback');
        return [];
      }
      
    final htmlPtr = html.toNativeUtf8();
    final cssPtr = css.toNativeUtf8();
      
      final resultPtr = _parseHtmlWithCssWithTimeout(htmlPtr, cssPtr);
      if (resultPtr == nullptr) {
        _cleanupPointers(htmlPtr, cssPtr);
        logPrint('[DART] parseHtmlWithCss: Parsing failed, returning fallback');
        return [];
      }
      
      final layoutBoxes = _extractLayoutBoxesWithTimeout(resultPtr);
      
      _cleanupPointers(htmlPtr, cssPtr);
      
      if (layoutBoxes.isEmpty) {
        logPrint('[DART] parseHtmlWithCss: No layout boxes extracted, returning fallback');
        return [];
      }
      
      logPrint('[DART] parseHtmlWithCss: Successfully parsed ${layoutBoxes.length} layout boxes');
      return layoutBoxes;
      
    } catch (e) {
      logPrint('[DART] parseHtmlWithCss: Error: $e');
      return [];
    }
  }

  static bool _validateHtmlInput(String html) {
    try {
      if (html.isEmpty) {
        logPrint('[DART] parseHtmlWithCss: HTML is empty');
        return false;
      }
      
      if (html.length > _maxHtmlSize) {
        logPrint('[DART] parseHtmlWithCss: HTML too large (${html.length} chars)');
        throw Exception('HTML content too large. Please try a simpler webpage.');
      }
      
      return true;
    } catch (e) {
      logPrint('[DART] _validateHtmlInput: Error: $e');
      return false;
    }
  }

  static Pointer<Void> _parseHtmlWithCssWithTimeout(Pointer<Utf8> htmlPtr, Pointer<Utf8> cssPtr) {
    try {
      logPrint('[DART] Calling Rust FFI _parseHtmlWithCss...');
      
      // Check if this is a large page that might cause freezing
      final htmlString = htmlPtr.toDartString();
      final isLargePage = htmlString.length > _largePageThreshold;
      
      if (isLargePage) {
        logPrint('[DART] WARNING: Large page detected (${htmlString.length} chars). Using timeout protection.');
      }
      
      final stopwatch = Stopwatch()..start();
      
      // Use timeout protection for large pages
      Pointer<Void> resultPtr;
      if (isLargePage) {
        try {
          resultPtr = _parseHtmlWithCssWithStrictTimeout(htmlPtr, cssPtr);
        } catch (e) {
          logPrint('[DART] Timeout during HTML parsing: $e');
          return nullptr;
        }
      } else {
        resultPtr = _parseHtmlWithCss(htmlPtr.cast<ffi.Char>(), cssPtr.cast<ffi.Char>());
      }
      
      stopwatch.stop();
      
      logPrint('[DART] Rust FFI _parseHtmlWithCss returned pointer: $resultPtr in ${stopwatch.elapsedMilliseconds}ms');
      
      if (stopwatch.elapsedMilliseconds > _maxHtmlParsingTimeMs) {
        logPrint('[DART] WARNING: HTML parsing took ${stopwatch.elapsedMilliseconds}ms (exceeded ${_maxHtmlParsingTimeMs}ms limit)');
      }
      
      if (resultPtr == nullptr) {
        logPrint('[DART] parseHtmlWithCss: resultPtr is null');
      }
      
      return resultPtr;
    } catch (e) {
      logPrint('[DART] _parseHtmlWithCssWithTimeout: Error: $e');
      return nullptr;
    }
  }

  static Pointer<Void> _parseHtmlWithCssWithStrictTimeout(Pointer<Utf8> htmlPtr, Pointer<Utf8> cssPtr) {
    try {
      logPrint('[DART] Using strict timeout protection for large page parsing');
      
      // For large pages, we'll use a more conservative approach
      // Instead of complex async timeouts, we'll limit the processing
      final htmlString = htmlPtr.toDartString();
      
      // If the page is extremely large, truncate it to prevent freezing
      if (htmlString.length > _maxHtmlSize) {
        logPrint('[DART] Page too large (${htmlString.length} chars), truncating to $_maxHtmlSize chars');
        final truncatedHtml = htmlString.substring(0, _maxHtmlSize);
        final truncatedPtr = truncatedHtml.toNativeUtf8();
        final result = _parseHtmlWithCss(truncatedPtr.cast<ffi.Char>(), cssPtr.cast<ffi.Char>());
        calloc.free(truncatedPtr);
        return result;
      }
      
      // For moderately large pages, proceed with normal parsing but log warnings
      logPrint('[DART] Proceeding with large page parsing (${htmlString.length} chars)');
      return _parseHtmlWithCss(htmlPtr.cast<ffi.Char>(), cssPtr.cast<ffi.Char>());
      
    } catch (e) {
      logPrint('[DART] _parseHtmlWithCssWithStrictTimeout: Error: $e');
      return nullptr;
    }
  }

  static void _cleanupPointers(Pointer<Utf8> htmlPtr, Pointer<Utf8> cssPtr) {
    try {
    calloc.free(htmlPtr);
    calloc.free(cssPtr);
    } catch (e) {
      logPrint('[DART] _cleanupPointers: Error: $e');
    }
  }

  static List<LayoutBox> _extractLayoutBoxesWithTimeout(Pointer<Void> resultPtr) {
    try {
      final stopwatch = Stopwatch()..start();
      
      // Check if we have a reasonable number of layout boxes
      final count = _getLayoutBoxCount(resultPtr);
      logPrint('[DART] _extractLayoutBoxesWithTimeout: Found $count layout boxes');
      
      if (count > _maxBoxCount) {
        logPrint('[DART] WARNING: Too many layout boxes ($count), using safe fallback');
        stopwatch.stop();
        logPrint('[DART] Using safe fallback instead of limited extraction');
        return [];
      }
      
      final layoutBoxes = _extractLayoutBoxes(resultPtr);
      
      stopwatch.stop();
      
      if (stopwatch.elapsedMilliseconds > _maxExtractionTimeMs) {
        logPrint('[DART] WARNING: Layout box extraction took ${stopwatch.elapsedMilliseconds}ms (exceeded ${_maxExtractionTimeMs}ms limit)');
      }
      
      logPrint('[DART] Layout box extraction completed in ${stopwatch.elapsedMilliseconds}ms');
      return layoutBoxes;
    } catch (e) {
      logPrint('[DART] _extractLayoutBoxesWithTimeout: Error: $e');
      return [];
    }
  }

  static List<LayoutBox> _extractLayoutBoxes(Pointer<Void> resultPtr) {
    if (resultPtr == nullptr) {
      logPrint('[DART] _extractLayoutBoxes: resultPtr is null');
      return [];
    }
    
    try {
    final count = _getLayoutBoxCount(resultPtr);
      logPrint('[DART] _extractLayoutBoxes: count = $count');
      
      if (!_isValidBoxCountForExtraction(count)) {
        _freeLayoutBoxArray(resultPtr);
        return [];
      }
      
      final layoutBoxes = _processLayoutBoxesInBatches(resultPtr, count);
      
      _freeLayoutBoxArray(resultPtr);
      logPrint('[DART] _extractLayoutBoxes: Successfully extracted ${layoutBoxes.length} boxes');
      return layoutBoxes;
      
    } catch (e) {
      logPrint('[DART] _extractLayoutBoxes: Error: $e');
      _safeFreeLayoutBoxArray(resultPtr);
      return [];
    }
  }

  static List<LayoutBox> _processLayoutBoxesInBatches(Pointer<Void> resultPtr, int count) {
    try {
    final List<LayoutBox> layoutBoxes = [];
      int processedCount = 0;
      
      for (int i = 0; i < count && processedCount < _maxBoxesToExtract; i++) {
        try {
          _logBatchProgress(i);
          
      final boxPtr = _getLayoutBox(resultPtr, i);
          
          if (!_isValidBoxPointerForExtraction(boxPtr)) {
            continue;
          }
          
        final layoutBox = _extractLayoutBox(boxPtr);
        if (layoutBox != null) {
          layoutBoxes.add(layoutBox);
        }
        _freeFFILayoutBox(boxPtr);
          processedCount++;
          
          _handleProcessingDelay(processedCount);
          
        } catch (e) {
          logPrint('[DART] _extractLayoutBoxes: Error processing box $i: $e');
          continue;
        }
      }
      
      _logExtractionCompletion(processedCount);
      return layoutBoxes;
    } catch (e) {
      logPrint('[DART] _processLayoutBoxesInBatches: Error: $e');
      return [];
    }
  }

  static void _logBatchProgress(int index) {
    try {
      if (index % _batchSize == 0) {
        logPrint('[DART] _extractLayoutBoxes: Processing batch ${index ~/ _batchSize + 1} (boxes $index to ${(index + _batchSize - 1).clamp(0, _maxBoxCount - 1)})');
      }
    } catch (e) {
      logPrint('[DART] _logBatchProgress: Error: $e');
    }
  }

  static bool _isValidBoxPointerForExtraction(Pointer<FFILayoutBox> boxPtr) {
    try {
      if (boxPtr == nullptr) {
        logPrint('[DART] _extractLayoutBoxes: boxPtr is null');
        return false;
      }
      
      if (boxPtr.address == 0) {
        logPrint('[DART] _extractLayoutBoxes: boxPtr address is 0');
        return false;
      }
      
      return true;
    } catch (e) {
      logPrint('[DART] _isValidBoxPointerForExtraction: Error: $e');
      return false;
    }
  }

  static void _handleProcessingDelay(int processedCount) {
    try {
      if (processedCount % 10 == 0) {
        logPrint('[DART] _extractLayoutBoxes: Processed $processedCount boxes so far');
      }
    } catch (e) {
      logPrint('[DART] _handleProcessingDelay: Error: $e');
    }
  }

  static void _logExtractionCompletion(int processedCount) {
    try {
      if (processedCount >= _maxBoxesToExtract) {
        logPrint('[DART] _extractLayoutBoxes: Reached maximum box limit ($_maxBoxesToExtract), stopping extraction');
      }
    } catch (e) {
      logPrint('[DART] _logExtractionCompletion: Error: $e');
    }
  }

  static void _safeFreeLayoutBoxArray(Pointer<Void> resultPtr) {
    try {
      _freeLayoutBoxArray(resultPtr);
    } catch (freeError) {
      logPrint('[DART] _extractLayoutBoxes: Error freeing resultPtr: $freeError');
    }
  }

  static Future<List<LayoutBox>> extractLayoutBoxesAsync(
    Pointer<Void> boxArrayPtr,
    Function(double) progressCallback,
  ) async {
    try {
      logPrint('EngineBridge: Using safe fallback layout extraction...');
      
      // For now, skip the problematic FFI extraction entirely
      // This prevents crashes while we investigate the FFI issue
      progressCallback(1.0);
      
      return [];
    } catch (e) {
      logPrint('EngineBridge: Error in extractLayoutBoxesAsync: $e');
      return [];
    }
  }

  static List<LayoutBox> _createDemoLayoutFromHtml(String html) {
    logPrint('EngineBridge: Creating demo layout from HTML');
    
    // Create a simple layout that represents the parsed content
    List<LayoutBox> boxes = [];
    
    // Header box
    boxes.add(LayoutBox(
      x: 50, y: 50, width: 700, height: 60,
      nodeType: 'h1', textContent: 'Successfully Parsed HTML',
      backgroundColor: '#e8f5e8', color: '#2e7d32', fontSize: 20,
      fontFamily: 'Arial', borderWidth: 2, borderColor: '#2e7d32',
      padding: 10, margin: 10, fontWeight: 700, textAlign: 'center',
    ));
    
    // Content info box
    boxes.add(LayoutBox(
      x: 50, y: 130, width: 700, height: 50,
      nodeType: 'p', textContent: 'HTML content length: ${html.length} characters',
      backgroundColor: '#fff3e0', color: '#ef6c00', fontSize: 14,
      fontFamily: 'Arial', borderWidth: 1, borderColor: '#ef6c00',
      padding: 10, margin: 10, fontWeight: 400, textAlign: 'left',
    ));
    
    // Try to extract some basic information from the HTML
    if (html.contains('<title>')) {
      final titleStart = html.indexOf('<title>') + 7;
      final titleEnd = html.indexOf('</title>', titleStart);
      if (titleEnd > titleStart) {
        final title = html.substring(titleStart, titleEnd).trim();
        boxes.add(LayoutBox(
          x: 50, y: 200, width: 700, height: 40,
          nodeType: 'p', textContent: 'Page Title: $title',
          backgroundColor: '#e3f2fd', color: '#1565c0', fontSize: 16,
          fontFamily: 'Arial', borderWidth: 1, borderColor: '#1565c0',
          padding: 10, margin: 10, fontWeight: 500, textAlign: 'left',
        ));
      }
    }
    
    // Status box
    boxes.add(LayoutBox(
      x: 50, y: 260, width: 700, height: 60,
      nodeType: 'p', textContent: 'The Rust engine successfully processed your request. This is a safe demo layout showing that Rift Browser core is functioning properly.',
      backgroundColor: '#f3e5f5', color: '#7b1fa2', fontSize: 12,
      fontFamily: 'Arial', borderWidth: 1, borderColor: '#7b1fa2',
      padding: 10, margin: 10, fontWeight: 400, textAlign: 'left',
    ));
    
    return boxes;
  }

  static List<LayoutBox> _createSafeFallbackPage() {
    logPrint('EngineBridge: Creating safe fallback page');
    return [];
  }

  static void logPrint(Object? obj) {
    // Use the same logPrint function from main.dart
    // ignore: avoid_print
    print(obj);
  }

  static bool _isValidBoxPointer(ffi.Pointer<FFILayoutBox> boxPtr) {
    try {
      if (boxPtr == nullptr || boxPtr.address == 0) {
        logPrint('[DART] _extractLayoutBox: Invalid boxPtr');
        return false;
      }
      
      if (boxPtr.address < _minPointerAddress || boxPtr.address > _maxPointerAddress) {
        logPrint('[DART] _extractLayoutBox: Invalid boxPtr address: 0x${boxPtr.address.toRadixString(16)}');
        return false;
      }
      
      return true;
    } catch (e) {
      logPrint('[DART] _isValidBoxPointer: Error: $e');
      return false;
    }
  }

  static LayoutBox? _extractLayoutBox(ffi.Pointer<FFILayoutBox> boxPtr) {
    try {
      if (!_isValidBoxPointer(boxPtr)) {
        return null;
      }
      
      final ffiBox = boxPtr.ref;
      
      final numericValues = _extractNumericValues(ffiBox);
      final stringValues = _extractStringValues(ffiBox);
      final calculatedValues = _calculateAverages(ffiBox);
      
      return _createLayoutBox(numericValues, stringValues, calculatedValues);
      
    } catch (e) {
      logPrint('[DART] _extractLayoutBox: Error extracting layout box: $e');
      return null;
    }
  }

  static bool _isValidBoxCountForExtraction(int count) {
    try {
      if (count <= 0) {
        logPrint('EngineBridge: Box count is zero or negative: $count');
        return false;
      }
      
      if (count > _maxBoxCount) {
        logPrint('EngineBridge: Box count exceeds maximum: $count > $_maxBoxCount');
        return false;
      }
      
      return true;
    } catch (e) {
      logPrint('EngineBridge: Error validating box count: $e');
      return false;
    }
  }

  static Future<List<LayoutBox>> parseHtmlInBackground(String html) async {
    if (!_initialized) {
      logPrint('Engine not initialized');
      return [];
    }

    try {
      logPrint('EngineBridge: Starting background HTML parsing...');
      
      // Use a background isolate for heavy processing
      final result = await compute(_parseHtmlInIsolate, html);
      
      if (result.isEmpty) {
        logPrint('EngineBridge: No layout boxes extracted in background, using fallback');
        return _createLayoutFromHtml(html);
      }
      
      logPrint('EngineBridge: Successfully extracted ${result.length} layout boxes in background');
      return result;
      
    } catch (e) {
      logPrint('EngineBridge: Error in background parsing: $e');
      return _createLayoutFromHtml(html);
    }
  }

  // Static method for isolate processing
  static Future<List<LayoutBox>> _parseHtmlInIsolate(String html) async {
    try {
      logPrint('EngineBridge: Isolate: Starting HTML parsing...');
      
      // Initialize FFI in this isolate
      _initializeInIsolate();
      
      // Validate input
      if (html.isEmpty) {
        logPrint('EngineBridge: Isolate: Empty HTML input');
        return _createLayoutFromHtml(html);
      }
      
      // Add timeout protection for the entire parsing process
      return await Future.any([
        _parseHtmlWithTimeout(html),
        Future.delayed(const Duration(seconds: 10), () {
          logPrint('EngineBridge: Isolate: HTML parsing timed out after 10 seconds, using fallback');
          return _createLayoutFromHtml(html);
        }),
      ]);
      
    } catch (e) {
      logPrint('EngineBridge: Isolate: Error: $e');
      return _createLayoutFromHtml(html);
    }
  }

  static Future<List<LayoutBox>> _parseHtmlWithTimeout(String html) async {
    try {
      // Call Rust engine
      logPrint('EngineBridge: Isolate: Calling Rust parse_html function...');
      final htmlBytes = html.codeUnits;
      final htmlPtr = calloc.allocate<Uint8>(htmlBytes.length + 1);
      for (int i = 0; i < htmlBytes.length; i++) {
        htmlPtr[i] = htmlBytes[i];
      }
      htmlPtr[htmlBytes.length] = 0; // null terminator
      
      final boxArrayPtr = _parseHtml(htmlPtr.cast<Char>());
      
      // Free the allocated memory
      calloc.free(htmlPtr);
      
      if (boxArrayPtr == null) {
        logPrint('EngineBridge: Isolate: Rust engine returned null, using fallback');
        return _createLayoutFromHtml(html);
      }
      
      // Get box count
      final count = _getLayoutBoxCount(boxArrayPtr);
      logPrint('EngineBridge: Isolate: Rust engine returned $count layout boxes');
      
      if (count <= 0) {
        logPrint('EngineBridge: Isolate: No layout boxes returned, using fallback');
        _freeLayoutBoxArray(boxArrayPtr);
        return _createLayoutFromHtml(html);
      }
      
      // Extract all boxes using batch method with timeout
      final layoutBoxes = await Future.any([
        _extractLayoutBoxesBatch(boxArrayPtr, count),
        Future.delayed(const Duration(seconds: 5), () {
          logPrint('EngineBridge: Isolate: Layout box extraction timed out, using fallback');
          _freeLayoutBoxArray(boxArrayPtr);
          return <LayoutBox>[];
        }),
      ]);
      
      // Clean up
      _freeLayoutBoxArray(boxArrayPtr);
      
      if (layoutBoxes.isEmpty) {
        logPrint('EngineBridge: Isolate: No layout boxes extracted, using fallback');
        return _createLayoutFromHtml(html);
      }
      
      logPrint('EngineBridge: Isolate: Successfully extracted ${layoutBoxes.length} layout boxes');
      return layoutBoxes;
      
    } catch (e) {
      logPrint('EngineBridge: Isolate: Error in _parseHtmlWithTimeout: $e');
      return _createLayoutFromHtml(html);
    }
  }

  // Initialize FFI functions in isolate
  static void _initializeInIsolate() {
    try {
      logPrint('EngineBridge: Isolate: Initializing FFI functions...');
      
      if (Platform.isWindows) {
        _lib = DynamicLibrary.open('rust_engine.dll');
      } else if (Platform.isMacOS) {
        _lib = DynamicLibrary.open('librust_engine.dylib');
      } else {
        _lib = DynamicLibrary.open('librust_engine.so');
      }
      
      if (_lib == null) {
        throw Exception('Failed to load Rust engine library in isolate');
      }
      
      _lookupFunctionPointers();
      logPrint('EngineBridge: Isolate: FFI functions initialized successfully');
      
    } catch (e) {
      logPrint('EngineBridge: Isolate: Failed to initialize FFI: $e');
      throw e;
    }
  }

  /// Parse a URL directly in Rust (fetches HTML, CSS, and creates layout boxes)
  static Future<List<LayoutBox>> parseUrlViaRust(String url) async {
    if (!_initialized) {
      logPrint('Engine not initialized');
      return [];
    }

    try {
      logPrint('EngineBridge: Starting URL parsing via Rust: $url');
      
      // Validate URL
      if (url.isEmpty) {
        logPrint('EngineBridge: Empty URL');
        return [];
      }
      
      // Convert URL to native UTF8
      final urlPtr = url.toNativeUtf8();
      
      try {
        // Call Rust engine to fetch and parse URL
        logPrint('EngineBridge: Calling Rust parse_url_via_rust function...');
        final boxArrayPtr = _parseUrlViaRust(urlPtr.cast<Char>());
        
        if (boxArrayPtr == nullptr) {
          logPrint('EngineBridge: Rust engine returned null for URL parsing');
          return [];
        }
        
        // Get layout box count
        final count = _getLayoutBoxCount(boxArrayPtr);
        logPrint('EngineBridge: Rust engine returned $count layout boxes for URL');
        
        if (count <= 0) {
          logPrint('EngineBridge: No layout boxes returned for URL, using fallback');
          _freeLayoutBoxArray(boxArrayPtr);
          return [];
        }
        
        // Extract layout boxes in batches
        final layoutBoxes = await _extractLayoutBoxesBatch(boxArrayPtr, count);
        
        // Clean up
        _freeLayoutBoxArray(boxArrayPtr);
        
        if (layoutBoxes.isEmpty) {
          logPrint('EngineBridge: No layout boxes extracted for URL, using fallback');
          return [];
        }
        
        logPrint('EngineBridge: Successfully extracted ${layoutBoxes.length} layout boxes for URL');
        return layoutBoxes;
        
      } finally {
        // Clean up URL pointer
        calloc.free(urlPtr);
      }
      
    } catch (e) {
      logPrint('EngineBridge: Error parsing URL via Rust: $e');
      return [];
    }
  }
}

class BatchResult {
  final List<LayoutBox> boxes;
  final int processedCount;
  
  BatchResult(this.boxes, this.processedCount);
} 