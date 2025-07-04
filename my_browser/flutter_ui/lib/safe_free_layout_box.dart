import 'dart:ffi';
import 'ffi_types.dart';
import 'ffi_structs.dart';

typedef FreeFFILayoutBoxDart = void Function(Pointer<FFILayoutBox>);

void safeFreeLayoutBox(Pointer<FFILayoutBox> boxPtr, FreeFFILayoutBoxDart freeFFILayoutBox) {
  try {
    if (boxPtr != nullptr) {
      freeFFILayoutBox(boxPtr);
    }
  } catch (e) {
    // Optionally log error
  }
} 