import 'models/layout_box.dart';
import 'engine_bridge.dart';

Future<List<LayoutBox>> parseUrlViaRust(String url) async {
  return await EngineBridge.parseUrlViaRust(url);
} 