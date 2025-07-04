import 'models/layout_box.dart';
import 'engine_bridge.dart';

Future<List<LayoutBox>> parseHtmlWithChunkedProcessing(String html) async {
  return await EngineBridge.parseHtmlInternalWithChunking(html);
} 