import 'dart:ffi';

String safeCStringToString(Pointer<Char> ptr, String defaultValue) {
  try {
    if (ptr == nullptr || ptr.address == 0) return defaultValue;
    final bytes = <int>[];
    int i = 0;
    while (true) {
      final byte = ptr.cast<Uint8>()[i];
      if (byte == 0) break;
      bytes.add(byte);
      i++;
    }
    return String.fromCharCodes(bytes);
  } catch (e) {
    return defaultValue;
  }
} 