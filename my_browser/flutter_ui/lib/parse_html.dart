import 'models/layout_box.dart';
import 'engine_bridge.dart';
import 'parse_html_with_chunked_processing.dart';

Future<List<LayoutBox>> parseHtml(String html) async {
  if (!EngineBridge.isInitialized) {
    EngineBridge.logPrint('Engine not initialized');
    return [];
  }

  // Validate that library and function pointers are properly loaded
  if (EngineBridge.lib == null) {
    EngineBridge.logPrint('EngineBridge: Dynamic library is null, reinitializing...');
    try {
      EngineBridge.initialize();
    } catch (e) {
      EngineBridge.logPrint('EngineBridge: Failed to reinitialize: $e');
      return EngineBridge.createLayoutFromHtml(html);
    }
  }

  try {
    EngineBridge.logPrint('EngineBridge: Starting HTML parsing...');
    if (html.isEmpty) {
      EngineBridge.logPrint('EngineBridge: Empty HTML input');
      return [];
    }
    final layoutBoxes = await parseHtmlWithChunkedProcessing(html);
    if (layoutBoxes.isEmpty) {
      EngineBridge.logPrint('EngineBridge: No layout boxes extracted, using fallback');
      return EngineBridge.createLayoutFromHtml(html);
    }
    EngineBridge.logPrint('EngineBridge: Successfully extracted \\${layoutBoxes.length} layout boxes from Rust engine');
    return layoutBoxes;
  } catch (e) {
    EngineBridge.logPrint('Error parsing HTML: $e');
    return EngineBridge.createLayoutFromHtml(html);
  }
} 