import 'ffi_structs.dart';

Map<String, double> extractNumericValues(FFILayoutBox ffiBox) {
  return {
    'x': ffiBox.x.isFinite ? ffiBox.x : 0.0,
    'y': ffiBox.y.isFinite ? ffiBox.y : 0.0,
    'width': ffiBox.width.isFinite && ffiBox.width >= 0 ? ffiBox.width : 0.0,
    'height': ffiBox.height.isFinite && ffiBox.height >= 0 ? ffiBox.height : 0.0,
    'font_size': ffiBox.font_size.isFinite && ffiBox.font_size > 0 ? ffiBox.font_size : 12.0,
    'font_weight': ffiBox.font_weight.isFinite && ffiBox.font_weight >= 0 ? ffiBox.font_weight : 400.0,
  };
} 