import 'models/layout_box.dart';
import 'dart:ffi';
import 'engine_bridge.dart';

Future<List<LayoutBox>> extractLayoutBoxesBatch(Pointer<Void> boxArrayPtr, int count) async {
  return await EngineBridge.extractLayoutBoxesBatch(boxArrayPtr, count);
} 