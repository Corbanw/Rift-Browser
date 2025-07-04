import 'dart:ffi';
import 'engine_bridge.dart' show DrawCommand, EngineBridge;

List<DrawCommand> extractDrawCommands(Pointer<Void> drawCommandsPtr) {
  return EngineBridge.extractDrawCommands(drawCommandsPtr);
} 