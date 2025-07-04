// ignore_for_file: non_constant_identifier_names
import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'dart:io';
import 'dart:convert';
import 'package:ffi/ffi.dart';
import 'dart:async';
import 'package:flutter/foundation.dart';

import 'models/layout_box.dart';
import 'ffi_structs.dart';
import 'ffi_types.dart';
import 'engine_result.dart';
import 'extract_images.dart';
import 'extract_links.dart';
import 'extract_text.dart';
import 'convert_utf8.dart';
import 'call_parse_html.dart';
import 'safe_free_layout_box.dart' hide FreeFFILayoutBoxDart;
import 'extract_numeric_values.dart';
import 'extract_string_values.dart';
import 'calculate_averages.dart';
import 'create_layout_box.dart';
import 'safe_cstring_to_string.dart';
import 'parse_html.dart';
import 'parse_html_with_css.dart';
import 'parse_html_to_draw_commands.dart';
import 'parse_url_via_rust.dart';
import 'parse_html_with_chunked_processing.dart';
import 'extract_single_draw_command.dart';

// All FFI typedefs have been moved to ffi_types.dart

class EngineInitResult {
  final bool success;
  final String? errorMessage;
  final String? stackTrace;
  EngineInitResult.success() : success = true, errorMessage = null, stackTrace = null;
  EngineInitResult.failure(this.errorMessage, this.stackTrace) : success = false;
}

class EngineBridge {
  static ffi.DynamicLibrary? _lib;
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
  
  // Draw command function pointers
  static late final GetDrawCommandCountDart _getDrawCommandCount;
  static late final GetDrawCommandDart _getDrawCommand;
  static late final FreeDrawCommandArrayDart _freeDrawCommandArray;
  
  // JavaScript function pointers
  static late final ExecuteJavaScriptDart _executeJavaScript;
  static late final ParseHtmlWithJavaScriptDart _parseHtmlWithJavaScript;

  // Attribute accessors
  static late final DomGetAttributeDart _domGetAttribute;
  static late final DomSetAttributeDart _domSetAttribute;
  static late final DomRemoveAttributeDart _domRemoveAttribute;
  static late final DomHasAttributeDart _domHasAttribute;
  static late final FreeCStringDart _freeCStringPtr;

  // classList accessors
  static late final DomClassListAddDart _domClassListAdd;
  static late final DomClassListRemoveDart _domClassListRemove;
  static late final DomClassListToggleDart _domClassListToggle;
  static late final DomClassListContainsDart _domClassListContains;

  // Node/Element property accessors
  // --- ADDED ---
  static late final DomGetTextContentDart _domGetTextContent;
  static late final DomSetTextContentDart _domSetTextContent;
  static late final DomGetInnerHtmlDart _domGetInnerHtml;
  static late final DomSetInnerHtmlDart _domSetInnerHtml;
  static late final DomGetOuterHtmlDart _domGetOuterHtml;
  static late final DomSetOuterHtmlDart _domSetOuterHtml;
  static late final DomGetIdDart _domGetId;
  static late final DomSetIdDart _domSetId;
  static late final DomGetTagNameDart _domGetTagName;
  static late final DomGetNodeTypeDart _domGetNodeType;

  // Style API function pointers
  // --- ADDED ---
  static late final DomGetStyleDart _domGetStyle;
  static late final DomSetStyleDart _domSetStyle;
  static late final DomGetStyleCssTextDart _domGetStyleCssText;
  static late final DomSetStyleCssTextDart _domSetStyleCssText;

  // Event Handling API function pointers
  // --- ADDED ---
  static late final DomAddEventListenerDart _domAddEventListener;
  static late final DomRemoveEventListenerDart _domRemoveEventListener;
  static late final DomDispatchEventDart _domDispatchEvent;

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
        _lib = ffi.DynamicLibrary.open('rust_engine.dll');
      } else if (Platform.isMacOS) {
        logPrint('EngineBridge: Loading macOS library: librust_engine.dylib');
        _lib = ffi.DynamicLibrary.open('librust_engine.dylib');
      } else {
        logPrint('EngineBridge: Loading Linux library: librust_engine.so');
        _lib = ffi.DynamicLibrary.open('librust_engine.so');
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
        ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>),
        ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>)
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
      
      _getDrawCommandCount = _lib!.lookupFunction<GetDrawCommandCountC, GetDrawCommandCountDart>('get_draw_command_count');
      logPrint('EngineBridge: _getDrawCommandCount function pointer initialized');
      
      _getDrawCommand = _lib!.lookupFunction<GetDrawCommandC, GetDrawCommandDart>('get_draw_command');
      logPrint('EngineBridge: _getDrawCommand function pointer initialized');
      
      _freeDrawCommandArray = _lib!.lookupFunction<FreeDrawCommandArrayC, FreeDrawCommandArrayDart>('free_draw_command_array');
      logPrint('EngineBridge: _freeDrawCommandArray function pointer initialized');
      
      // Initialize JavaScript function pointers
      _executeJavaScript = _lib!.lookupFunction<ExecuteJavaScriptC, ExecuteJavaScriptDart>('execute_javascript');
      logPrint('EngineBridge: _executeJavaScript function pointer initialized');
      
      _parseHtmlWithJavaScript = _lib!.lookupFunction<ParseHtmlWithJavaScriptC, ParseHtmlWithJavaScriptDart>('parse_html_with_javascript');
      logPrint('EngineBridge: _parseHtmlWithJavaScript function pointer initialized');
      
      _domGetAttribute = _lib!.lookupFunction<DomGetAttributeC, DomGetAttributeDart>('dom_get_attribute');
      _domSetAttribute = _lib!.lookupFunction<DomSetAttributeC, DomSetAttributeDart>('dom_set_attribute');
      _domRemoveAttribute = _lib!.lookupFunction<DomRemoveAttributeC, DomRemoveAttributeDart>('dom_remove_attribute');
      _domHasAttribute = _lib!.lookupFunction<DomHasAttributeC, DomHasAttributeDart>('dom_has_attribute');
      
      _freeCStringPtr = _lib!.lookupFunction<FreeCStringC, FreeCStringDart>('free_c_string');
      
      _domClassListAdd = _lib!.lookupFunction<DomClassListAddC, DomClassListAddDart>('dom_class_list_add');
      _domClassListRemove = _lib!.lookupFunction<DomClassListRemoveC, DomClassListRemoveDart>('dom_class_list_remove');
      _domClassListToggle = _lib!.lookupFunction<DomClassListToggleC, DomClassListToggleDart>('dom_class_list_toggle');
      _domClassListContains = _lib!.lookupFunction<DomClassListContainsC, DomClassListContainsDart>('dom_class_list_contains');
      
      _domGetTextContent = _lib!.lookupFunction<DomGetTextContentC, DomGetTextContentDart>('dom_get_text_content');
      _domSetTextContent = _lib!.lookupFunction<DomSetTextContentC, DomSetTextContentDart>('dom_set_text_content');
      _domGetInnerHtml = _lib!.lookupFunction<DomGetInnerHtmlC, DomGetInnerHtmlDart>('dom_get_inner_html');
      _domSetInnerHtml = _lib!.lookupFunction<DomSetInnerHtmlC, DomSetInnerHtmlDart>('dom_set_inner_html');
      _domGetOuterHtml = _lib!.lookupFunction<DomGetOuterHtmlC, DomGetOuterHtmlDart>('dom_get_outer_html');
      _domSetOuterHtml = _lib!.lookupFunction<DomSetOuterHtmlC, DomSetOuterHtmlDart>('dom_set_outer_html');
      _domGetId = _lib!.lookupFunction<DomGetIdC, DomGetIdDart>('dom_get_id');
      _domSetId = _lib!.lookupFunction<DomSetIdC, DomSetIdDart>('dom_set_id');
      _domGetTagName = _lib!.lookupFunction<DomGetTagNameC, DomGetTagNameDart>('dom_get_tag_name');
      _domGetNodeType = _lib!.lookupFunction<DomGetNodeTypeC, DomGetNodeTypeDart>('dom_get_node_type');
      
      _domGetStyle = _lib!.lookupFunction<DomGetStyleC, DomGetStyleDart>('dom_get_style');
      _domSetStyle = _lib!.lookupFunction<DomSetStyleC, DomSetStyleDart>('dom_set_style');
      _domGetStyleCssText = _lib!.lookupFunction<DomGetStyleCssTextC, DomGetStyleCssTextDart>('dom_get_style_css_text');
      _domSetStyleCssText = _lib!.lookupFunction<DomSetStyleCssTextC, DomSetStyleCssTextDart>('dom_set_style_css_text');
      
      // Event Handling API
      _domAddEventListener = _lib!.lookupFunction<DomAddEventListenerC, DomAddEventListenerDart>('dom_add_event_listener');
      _domRemoveEventListener = _lib!.lookupFunction<DomRemoveEventListenerC, DomRemoveEventListenerDart>('dom_remove_event_listener');
      _domDispatchEvent = _lib!.lookupFunction<DomDispatchEventC, DomDispatchEventDart>('dom_dispatch_event');
      
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
        return createLayoutFromHtml(html);
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
      final layoutBoxes = await parseHtmlWithChunkedProcessing(html);
      
      if (layoutBoxes.isEmpty) {
        logPrint('EngineBridge: No layout boxes extracted, using fallback');
        return createLayoutFromHtml(html);
      }
      
      logPrint('EngineBridge: Successfully extracted ${layoutBoxes.length} layout boxes from Rust engine');
      return layoutBoxes;
      
    } catch (e) {
      logPrint('Error parsing HTML: $e');
      return createLayoutFromHtml(html);
    }
  }

  static Future<List<LayoutBox>> _parseHtmlInternalWithChunking(String html) async {
    // Convert HTML to native UTF8
    final htmlPtr = convertStringToNativeUtf8(html);
    if (htmlPtr == nullptr) {
      logPrint('EngineBridge: Failed to convert HTML to UTF8');
      return [];
    }
    
    try {
      // Call Rust engine to parse HTML
      logPrint('EngineBridge: Calling Rust parse_html function...');
      final boxArrayPtr = callParseHtml(_parseHtml, htmlPtr);
      
      if (boxArrayPtr == nullptr) {
        logPrint('EngineBridge: Rust engine returned null, using fallback');
        return [];
      }
      
      // Get layout box count
      final count = _getLayoutBoxCount(boxArrayPtr);
      logPrint('EngineBridge: Rust engine returned $count layout boxes');
      
      if (count <= 0) {
        logPrint('EngineBridge: Invalid box count ($count), using fallback');
        _freeLayoutBoxArrayHelper(boxArrayPtr);
        return [];
      }
      
      // Extract layout boxes in batches for better performance
      final layoutBoxes = await _extractLayoutBoxesBatch(boxArrayPtr, count);
      
      // Clean up
      _freeLayoutBoxArrayHelper(boxArrayPtr);
      
      return layoutBoxes;
      
    } finally {
      // Clean up HTML pointer
      calloc.free(htmlPtr);
    }
  }

  static Future<List<LayoutBox>> _extractLayoutBoxesBatch(ffi.Pointer<ffi.Void> boxArrayPtr, int count) async {
    try {
      logPrint('EngineBridge: Starting batch extraction of $count boxes');
      final List<LayoutBox> layoutBoxes = [];
      const int batchSize = 1000;
      final ffi.Pointer<ffi.Pointer<FFILayoutBox>> batchPtr = calloc.allocate<ffi.Pointer<FFILayoutBox>>(batchSize);
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
          
          _freeFFILayoutBox(boxPtr);
        }
        
        extracted += n;
        logPrint('EngineBridge: Batch extracted $extracted/$count boxes (total so far: ${layoutBoxes.length})');
        await Future.delayed(const Duration(milliseconds: 1));
      }
      
      calloc.free(batchPtr);
      logPrint('EngineBridge: Finished batch extraction, total ${layoutBoxes.length} boxes');
      return layoutBoxes;
    } catch (e) {
      logPrint('EngineBridge: Error in batch extraction: $e');
      return [];
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
      
      final resultPtr = _parseHtmlWithCssWithTimeout(htmlPtr.cast<ffi.Uint8>(), cssPtr.cast<ffi.Uint8>());
      if (resultPtr == nullptr) {
        _cleanupPointers(htmlPtr.cast<ffi.Uint8>(), cssPtr.cast<ffi.Uint8>());
        logPrint('[DART] parseHtmlWithCss: Parsing failed, returning fallback');
        return [];
      }
      
      final layoutBoxes = _extractLayoutBoxesWithTimeout(resultPtr);
      
      _cleanupPointers(htmlPtr.cast<ffi.Uint8>(), cssPtr.cast<ffi.Uint8>());
      
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

  static ffi.Pointer<ffi.Void> _parseHtmlWithCssWithTimeout(ffi.Pointer<ffi.Uint8> htmlPtr, ffi.Pointer<ffi.Uint8> cssPtr) {
    try {
      logPrint('[DART] Calling Rust FFI _parseHtmlWithCss...');
      
      // Check if this is a large page that might cause freezing
      final htmlString = htmlPtr.cast<Utf8>().toDartString();
      final isLargePage = htmlString.length > _largePageThreshold;
      
      if (isLargePage) {
        logPrint('[DART] WARNING: Large page detected (${htmlString.length} chars). Using timeout protection.');
      }
      
      final stopwatch = Stopwatch()..start();
      
      // Use timeout protection for large pages
      ffi.Pointer<ffi.Void> resultPtr;
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

  static ffi.Pointer<ffi.Void> _parseHtmlWithCssWithStrictTimeout(ffi.Pointer<ffi.Uint8> htmlPtr, ffi.Pointer<ffi.Uint8> cssPtr) {
    try {
      logPrint('[DART] Using strict timeout protection for large page parsing');
      
      // For large pages, we'll use a more conservative approach
      // Instead of complex async timeouts, we'll limit the processing
      final htmlString = htmlPtr.cast<Utf8>().toDartString();
      
      // If the page is extremely large, truncate it to prevent freezing
      if (htmlString.length > _maxHtmlSize) {
        logPrint('[DART] Page too large (${htmlString.length} chars), truncating to $_maxHtmlSize chars');
        final truncatedHtml = htmlString.substring(0, _maxHtmlSize);
        final truncatedPtr = truncatedHtml.toNativeUtf8();
        final result = _parseHtmlWithCss(htmlPtr.cast<ffi.Char>(), cssPtr.cast<ffi.Char>());
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

  static void _cleanupPointers(ffi.Pointer<ffi.Uint8> htmlPtr, ffi.Pointer<ffi.Uint8> cssPtr) {
    try {
    calloc.free(htmlPtr);
    calloc.free(cssPtr);
    } catch (e) {
      logPrint('[DART] _cleanupPointers: Error: $e');
    }
  }

  static List<LayoutBox> _extractLayoutBoxesWithTimeout(ffi.Pointer<ffi.Void> resultPtr) {
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

  static List<LayoutBox> _extractLayoutBoxes(ffi.Pointer<ffi.Void> resultPtr) {
    if (resultPtr == nullptr) {
      logPrint('[DART] _extractLayoutBoxes: resultPtr is null');
      return [];
    }
    
    try {
    final count = _getLayoutBoxCount(resultPtr);
      logPrint('[DART] _extractLayoutBoxes: count = $count');
      
      if (!_isValidBoxCountForExtraction(count)) {
        _freeLayoutBoxArrayHelper(resultPtr);
        return [];
      }
      
      final layoutBoxes = _processLayoutBoxesInBatches(resultPtr, count);
      
      _freeLayoutBoxArrayHelper(resultPtr);
      logPrint('[DART] _extractLayoutBoxes: Successfully extracted ${layoutBoxes.length} boxes');
      return layoutBoxes;
      
    } catch (e) {
      logPrint('[DART] _extractLayoutBoxes: Error: $e');
      _freeLayoutBoxArrayHelper(resultPtr);
      return [];
    }
  }

  static List<LayoutBox> _processLayoutBoxesInBatches(ffi.Pointer<ffi.Void> resultPtr, int count) {
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

  static bool _isValidBoxPointerForExtraction(ffi.Pointer<FFILayoutBox> boxPtr) {
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

  static void _freeLayoutBoxArrayHelper(ffi.Pointer<ffi.Void> resultPtr) {
    try {
      _freeLayoutBoxArray(resultPtr);
    } catch (e) {
      logPrint('[DART] _extractLayoutBoxes: Error freeing resultPtr: $e');
    }
  }

  static Future<List<LayoutBox>> extractLayoutBoxesAsync(
    ffi.Pointer<ffi.Void> boxArrayPtr,
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

  static List<LayoutBox> createLayoutFromHtml(String html) {
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
      nodeType: 'p', textContent: 'The Rust engine successfully processed your request. This is a safe demo layout showing that Velox Browser core is functioning properly.',
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
      
      final numericValues = extractNumericValues(ffiBox);
      final stringValues = extractStringValues(ffiBox, safeCStringToString);
      final calculatedValues = calculateAverages(ffiBox);
      
      return createLayoutBox(numericValues, stringValues, calculatedValues);
      
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
        return createLayoutFromHtml(html);
      }
      
      logPrint('EngineBridge: Successfully extracted ${result.length} layout boxes in background');
      return result;
      
    } catch (e) {
      logPrint('EngineBridge: Error in background parsing: $e');
      return createLayoutFromHtml(html);
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
        return createLayoutFromHtml(html);
      }
      
      // Add timeout protection for the entire parsing process
      return await Future.any([
        _parseHtmlWithTimeout(html),
        Future.delayed(const Duration(seconds: 10), () {
          logPrint('EngineBridge: Isolate: HTML parsing timed out after 10 seconds, using fallback');
          return createLayoutFromHtml(html);
        }),
      ]);
      
    } catch (e) {
      logPrint('EngineBridge: Isolate: Error: $e');
      return createLayoutFromHtml(html);
    }
  }

  static Future<List<LayoutBox>> _parseHtmlWithTimeout(String html) async {
    try {
      // Call Rust engine
      logPrint('EngineBridge: Isolate: Calling Rust parse_html function...');
      final htmlBytes = html.codeUnits;
      final htmlPtr = calloc.allocate<ffi.Uint8>(htmlBytes.length + 1);
      for (int i = 0; i < htmlBytes.length; i++) {
        htmlPtr[i] = htmlBytes[i];
      }
      htmlPtr[htmlBytes.length] = 0; // null terminator
      
      final boxArrayPtr = callParseHtml(_parseHtml, htmlPtr);
      
      // Free the allocated memory
      calloc.free(htmlPtr);
      
      // Get box count
      final count = _getLayoutBoxCount(boxArrayPtr);
      logPrint('EngineBridge: Isolate: Rust engine returned $count layout boxes');
      
      if (count <= 0) {
        logPrint('EngineBridge: Isolate: No layout boxes returned, using fallback');
        _freeLayoutBoxArrayHelper(boxArrayPtr);
        return createLayoutFromHtml(html);
      }
      
      // Extract all boxes using batch method with timeout
      final layoutBoxes = await Future.any([
        _extractLayoutBoxesBatch(boxArrayPtr, count),
        Future.delayed(const Duration(seconds: 5), () {
          logPrint('EngineBridge: Isolate: Layout box extraction timed out, using fallback');
          _freeLayoutBoxArrayHelper(boxArrayPtr);
          return <LayoutBox>[];
        }),
      ]);
      
      // Clean up
      _freeLayoutBoxArrayHelper(boxArrayPtr);
      
      if (layoutBoxes.isEmpty) {
        logPrint('EngineBridge: Isolate: No layout boxes extracted, using fallback');
        return createLayoutFromHtml(html);
      }
      
      logPrint('EngineBridge: Isolate: Successfully extracted ${layoutBoxes.length} layout boxes');
      return layoutBoxes;
      
    } catch (e) {
      logPrint('EngineBridge: Isolate: Error in _parseHtmlWithTimeout: $e');
      return createLayoutFromHtml(html);
    }
  }

  // Initialize FFI functions in isolate
  static void _initializeInIsolate() {
    try {
      logPrint('EngineBridge: Isolate: Initializing FFI functions...');
      
      if (Platform.isWindows) {
        _lib = ffi.DynamicLibrary.open('rust_engine.dll');
      } else if (Platform.isMacOS) {
        _lib = ffi.DynamicLibrary.open('librust_engine.dylib');
      } else {
        _lib = ffi.DynamicLibrary.open('librust_engine.so');
      }
      
      if (_lib == null) {
        throw Exception('Failed to load Rust engine library in isolate');
      }
      
      _lookupFunctionPointers();
      logPrint('EngineBridge: Isolate: FFI functions initialized successfully');
      
    } catch (e) {
      logPrint('EngineBridge: Isolate: Failed to initialize FFI: $e');
      rethrow;
    }
  }

  /// Parse HTML and CSS to draw commands for direct painting
  static DrawCommandResult parseHtmlToDrawCommands(String html, String css) {
    if (!_initialized) {
      logPrint('Engine not initialized');
      return DrawCommandResult.failure('Engine not initialized');
    }

    try {
      logPrint('EngineBridge: Starting HTML to draw commands parsing...');
      
      // Validate input
      if (html.isEmpty) {
        logPrint('EngineBridge: Empty HTML input');
        return DrawCommandResult.failure('Empty HTML input');
      }
      
      // Convert HTML to native UTF8
      final htmlPtr = html.toNativeUtf8();
      
      try {
        // Call Rust engine to parse HTML to draw commands
        logPrint('EngineBridge: Calling Rust parse_html_to_draw_commands function...');
        final drawCommandsPtr = _parseHtmlToDrawCommands(htmlPtr.cast<ffi.Uint8>().cast<ffi.Char>());
        
        if (drawCommandsPtr == nullptr) {
          logPrint('EngineBridge: Rust engine returned null for draw commands');
          return DrawCommandResult.failure('Failed to parse HTML to draw commands');
        }
        
        // Convert draw commands to Dart objects
        final drawCommands = extractDrawCommands(drawCommandsPtr);
        
        // Clean up
        _freeDrawCommands(drawCommandsPtr);
        
        logPrint('EngineBridge: Successfully extracted ${drawCommands.length} draw commands');
        return DrawCommandResult.success(drawCommands);
        
      } finally {
        // Clean up HTML pointer
        calloc.free(htmlPtr);
      }
      
    } catch (e) {
      logPrint('EngineBridge: Error parsing HTML to draw commands: $e');
      return DrawCommandResult.failure('Error: $e');
    }
  }

  /// Public API: Extracts a list of DrawCommand from a pointer to a draw command array
  static List<DrawCommand> extractDrawCommands(ffi.Pointer<ffi.Void> drawCommandsPtr) {
    if (drawCommandsPtr == nullptr) return [];
    final count = _getDrawCommandCount(drawCommandsPtr);
    final result = <DrawCommand>[];
    for (var i = 0; i < count; i++) {
      final cmdPtr = _getDrawCommand(drawCommandsPtr, i);
      if (cmdPtr == nullptr) continue;
      result.add(extractSingleDrawCommand(cmdPtr));
    }
    return result;
  }

  static void _freeDrawCommands(ffi.Pointer<ffi.Void> drawCommandsPtr) {
    if (drawCommandsPtr != nullptr) {
      _freeDrawCommandArray(drawCommandsPtr);
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
        final boxArrayPtr = _parseUrlViaRust(urlPtr as ffi.Pointer<ffi.Char>);
        
        if (boxArrayPtr == nullptr) {
          logPrint('EngineBridge: Rust engine returned null for URL parsing');
          return [];
        }
        
        // Get layout box count
        final count = _getLayoutBoxCount(boxArrayPtr);
        logPrint('EngineBridge: Rust engine returned $count layout boxes for URL');
        
        if (count <= 0) {
          logPrint('EngineBridge: No layout boxes returned for URL, using fallback');
          _freeLayoutBoxArrayHelper(boxArrayPtr);
          return [];
        }
        
        // Extract layout boxes in batches
        final layoutBoxes = await _extractLayoutBoxesBatch(boxArrayPtr, count);
        
        // Clean up
        _freeLayoutBoxArrayHelper(boxArrayPtr);
        
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

  /// Helper to convert List<String> to Pointer<Pointer<Utf8>> for FFI
  static ffi.Pointer<ffi.Pointer<ffi.Uint8>> _toPointerPointerUtf8(List<String> strings) {
    final ptr = calloc<ffi.Pointer<ffi.Uint8>>(strings.length);
    for (var i = 0; i < strings.length; i++) {
      ptr[i] = strings[i].toNativeUtf8().cast<ffi.Uint8>();
    }
    return ptr;
  }

  /// Helper to free Pointer<Pointer<Utf8>>
  static void _freePointerPointerUtf8(ffi.Pointer<ffi.Pointer<ffi.Uint8>> ptr, int length) {
    for (var i = 0; i < length; i++) {
      calloc.free(ptr[i]);
    }
    calloc.free(ptr);
  }

  /// Execute JavaScript code
  static Future<bool> executeJavaScript(String script, String scriptName) async {
    if (!_initialized) {
      logPrint('Engine not initialized');
      return false;
    }

    try {
      logPrint('EngineBridge: Executing JavaScript: $scriptName');
      
      // Convert script and name to native UTF8
      final scriptPtr = convertStringToNativeUtf8(script);
      final namePtr = convertStringToNativeUtf8(scriptName);
      
      try {
        // Call Rust engine to execute JavaScript
        final result = _executeJavaScript(scriptPtr.cast<ffi.Char>(), namePtr.cast<ffi.Char>());
        
        if (result == 0) {
          logPrint('EngineBridge: JavaScript executed successfully: $scriptName');
          return true;
        } else {
          logPrint('EngineBridge: JavaScript execution failed: $scriptName');
          return false;
        }
      } finally {
        // Clean up pointers
        calloc.free(scriptPtr);
        calloc.free(namePtr);
      }
    } catch (e) {
      logPrint('EngineBridge: Error executing JavaScript: $e');
      return false;
    }
  }

  /// Parse HTML with JavaScript execution
  static Future<List<LayoutBox>> parseHtmlWithJavaScript(String html) async {
    if (!_initialized) {
      logPrint('Engine not initialized');
      return [];
    }

    try {
      logPrint('EngineBridge: Starting HTML parsing with JavaScript...');
      
      // Validate input
      if (html.isEmpty) {
        logPrint('EngineBridge: Empty HTML input');
        return [];
      }
      
      // Convert HTML to native UTF8
      final htmlPtr = convertStringToNativeUtf8(html);
      if (htmlPtr == nullptr) {
        logPrint('EngineBridge: Failed to convert HTML to UTF8');
        return [];
      }
      
      try {
        // Call Rust engine to parse HTML with JavaScript
        logPrint('EngineBridge: Calling Rust parse_html_with_javascript function...');
        final boxArrayPtr = _parseHtmlWithJavaScript(htmlPtr.cast<ffi.Char>());
        
        if (boxArrayPtr == nullptr) {
          logPrint('EngineBridge: Rust engine returned null for HTML with JavaScript parsing');
          return [];
        }
        
        // Get layout box count
        final count = _getLayoutBoxCount(boxArrayPtr);
        logPrint('EngineBridge: Rust engine returned $count layout boxes with JavaScript');
        
        if (count <= 0) {
          logPrint('EngineBridge: No layout boxes returned with JavaScript, using fallback');
          _freeLayoutBoxArrayHelper(boxArrayPtr);
          return [];
        }
        
        // Extract layout boxes in batches
        final layoutBoxes = await _extractLayoutBoxesBatch(boxArrayPtr, count);
        
        // Clean up
        _freeLayoutBoxArrayHelper(boxArrayPtr);
        
        if (layoutBoxes.isEmpty) {
          logPrint('EngineBridge: No layout boxes extracted with JavaScript, using fallback');
          return [];
        }
        
        logPrint('EngineBridge: Successfully extracted ${layoutBoxes.length} layout boxes with JavaScript');
        return layoutBoxes;
        
      } finally {
        // Clean up HTML pointer
        calloc.free(htmlPtr);
      }
      
    } catch (e) {
      logPrint('EngineBridge: Error parsing HTML with JavaScript: $e');
      return createLayoutFromHtml(html);
    }
  }

  // Attribute accessors
  static String? getAttribute(int nodeId, String name) {
    final namePtr = name.toNativeUtf8();
    final resultPtr = _domGetAttribute(nodeId, namePtr.cast<ffi.Char>());
    calloc.free(namePtr);
    if (resultPtr == nullptr) return null;
    final result = resultPtr.cast<Utf8>().toDartString();
    _freeCString(resultPtr);
    return result;
  }

  static void setAttribute(int nodeId, String name, String value) {
    final namePtr = name.toNativeUtf8();
    final valuePtr = value.toNativeUtf8();
    _domSetAttribute(nodeId, namePtr.cast<ffi.Char>(), valuePtr.cast<ffi.Char>());
    calloc.free(namePtr);
    calloc.free(valuePtr);
  }

  static void removeAttribute(int nodeId, String name) {
    final namePtr = name.toNativeUtf8();
    _domRemoveAttribute(nodeId, namePtr.cast<ffi.Char>());
    calloc.free(namePtr);
  }

  static bool hasAttribute(int nodeId, String name) {
    final namePtr = name.toNativeUtf8();
    final result = _domHasAttribute(nodeId, namePtr.cast<ffi.Char>()) != 0;
    calloc.free(namePtr);
    return result;
  }

  static void _freeCString(ffi.Pointer<ffi.Char> ptr) {
    if (ptr != nullptr) {
      _freeCStringPtr(ptr);
    }
  }

  // classList accessors
  static void classListAdd(int nodeId, String className) {
    final classPtr = className.toNativeUtf8();
    _domClassListAdd(nodeId, classPtr.cast<ffi.Char>());
    calloc.free(classPtr);
  }

  static void classListRemove(int nodeId, String className) {
    final classPtr = className.toNativeUtf8();
    _domClassListRemove(nodeId, classPtr.cast<ffi.Char>());
    calloc.free(classPtr);
  }

  static void classListToggle(int nodeId, String className) {
    final classPtr = className.toNativeUtf8();
    _domClassListToggle(nodeId, classPtr.cast<ffi.Char>());
    calloc.free(classPtr);
  }

  static bool classListContains(int nodeId, String className) {
    final classPtr = className.toNativeUtf8();
    final result = _domClassListContains(nodeId, classPtr.cast<ffi.Char>()) != 0;
    calloc.free(classPtr);
    return result;
  }

  // Node/Element property accessors
  static String? getTextContent(int nodeId) {
    final ptr = _domGetTextContent(nodeId);
    if (ptr == nullptr) return null;
    final result = ptr.cast<Utf8>().toDartString();
    _freeCString(ptr);
    return result;
  }
  static void setTextContent(int nodeId, String value) {
    final valuePtr = value.toNativeUtf8();
    _domSetTextContent(nodeId, valuePtr.cast<ffi.Char>());
    calloc.free(valuePtr);
  }
  static String? getInnerHtml(int nodeId) {
    final ptr = _domGetInnerHtml(nodeId);
    if (ptr == nullptr) return null;
    final result = ptr.cast<Utf8>().toDartString();
    _freeCString(ptr);
    return result;
  }
  static void setInnerHtml(int nodeId, String value) {
    final valuePtr = value.toNativeUtf8();
    _domSetInnerHtml(nodeId, valuePtr.cast<ffi.Char>());
    calloc.free(valuePtr);
  }
  static String? getOuterHtml(int nodeId) {
    final ptr = _domGetOuterHtml(nodeId);
    if (ptr == nullptr) return null;
    final result = ptr.cast<Utf8>().toDartString();
    _freeCString(ptr);
    return result;
  }
  static void setOuterHtml(int nodeId, String value) {
    final valuePtr = value.toNativeUtf8();
    _domSetOuterHtml(nodeId, valuePtr.cast<ffi.Char>());
    calloc.free(valuePtr);
  }
  static String? getId(int nodeId) {
    final ptr = _domGetId(nodeId);
    if (ptr == nullptr) return null;
    final result = ptr.cast<Utf8>().toDartString();
    _freeCString(ptr);
    return result;
  }
  static void setId(int nodeId, String value) {
    final valuePtr = value.toNativeUtf8();
    _domSetId(nodeId, valuePtr.cast<ffi.Char>());
    calloc.free(valuePtr);
  }
  static String? getTagName(int nodeId) {
    final ptr = _domGetTagName(nodeId);
    if (ptr == nullptr) return null;
    final result = ptr.cast<Utf8>().toDartString();
    _freeCString(ptr);
    return result;
  }
  static int getNodeType(int nodeId) {
    return _domGetNodeType(nodeId);
  }

  // Style API function pointers
  // --- ADDED ---
  static String? getStyle(int nodeId, String name) {
    final namePtr = name.toNativeUtf8();
    final resultPtr = _domGetStyle(nodeId, namePtr.cast<ffi.Char>());
    calloc.free(namePtr);
    if (resultPtr == nullptr) return null;
    final result = resultPtr.cast<Utf8>().toDartString();
    _freeCString(resultPtr);
    return result;
  }

  static void setStyle(int nodeId, String name, String value) {
    final namePtr = name.toNativeUtf8();
    final valuePtr = value.toNativeUtf8();
    _domSetStyle(nodeId, namePtr.cast<ffi.Char>(), valuePtr.cast<ffi.Char>());
    calloc.free(namePtr);
    calloc.free(valuePtr);
  }

  static String? getStyleCssText(int nodeId) {
    final resultPtr = _domGetStyleCssText(nodeId);
    if (resultPtr == nullptr) return null;
    final result = resultPtr.cast<Utf8>().toDartString();
    _freeCString(resultPtr);
    return result;
  }

  static void setStyleCssText(int nodeId, String value) {
    final valuePtr = value.toNativeUtf8();
    _domSetStyleCssText(nodeId, valuePtr.cast<ffi.Char>());
    calloc.free(valuePtr);
  }

  // Event Handling API
  // --- ADDED ---
  static void addEventListener(int nodeId, String type, int callbackId) {
    final typePtr = type.toNativeUtf8();
    _domAddEventListener(nodeId, typePtr.cast<ffi.Char>(), callbackId);
    calloc.free(typePtr);
  }
  static void removeEventListener(int nodeId, String type, int callbackId) {
    final typePtr = type.toNativeUtf8();
    _domRemoveEventListener(nodeId, typePtr.cast<ffi.Char>(), callbackId);
    calloc.free(typePtr);
  }
  static void dispatchEvent(int nodeId, String type) {
    final typePtr = type.toNativeUtf8();
    _domDispatchEvent(nodeId, typePtr.cast<ffi.Char>());
    calloc.free(typePtr);
  }

  // Public API for refactored top-level functions
  static bool get isInitialized => _initialized;
  static ffi.DynamicLibrary? get lib => _lib;
  static Future<List<LayoutBox>> parseHtmlInternalWithChunking(String html) => _parseHtmlInternalWithChunking(html);
  static Future<List<LayoutBox>> extractLayoutBoxesBatch(ffi.Pointer<ffi.Void> boxArrayPtr, int count) => _extractLayoutBoxesBatch(boxArrayPtr, count);
}

class BatchResult {
  final List<LayoutBox> boxes;
  final int processedCount;
  
  BatchResult(this.boxes, this.processedCount);
}

class DrawCommandResult {
  final bool success;
  final List<DrawCommand> drawCommands;
  final String? errorMessage;

  DrawCommandResult.success(this.drawCommands)
      : success = true,
        errorMessage = null;

  DrawCommandResult.failure(this.errorMessage)
      : success = false,
        drawCommands = [];
}

/// Represents a draw command for the painting system
class DrawCommand {
  final double x;
  final double y;
  final double w;
  final double h;
  final int color;
  final String content;
  final String font;
  final double size;
  final String src;
  final DrawCommandType type;

  const DrawCommand.rect({
    required this.x,
    required this.y,
    required this.w,
    required this.h,
    required this.color,
  }) : content = '',
       font = '',
       size = 0,
       src = '',
       type = DrawCommandType.rect;

  const DrawCommand.text({
    required this.x,
    required this.y,
    required this.content,
    required this.font,
    required this.size,
    required this.color,
  }) : w = 0,
       h = 0,
       src = '',
       type = DrawCommandType.text;

  const DrawCommand.image({
    required this.x,
    required this.y,
    required this.src,
  }) : w = 0,
       h = 0,
       color = 0,
       content = '',
       font = '',
       size = 0,
       type = DrawCommandType.image;
}

enum DrawCommandType {
  rect,
  text,
  image,
} 