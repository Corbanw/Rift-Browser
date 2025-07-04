import 'ffi_structs.dart';

double calculateAverage(List<double> values) {
  final validValues = values.where((v) => v.isFinite && v >= 0).toList();
  if (validValues.isEmpty) return 0.0;
  return validValues.reduce((a, b) => a + b) / validValues.length;
}

Map<String, double> calculateAverages(FFILayoutBox ffiBox) {
  return {
    'margin': calculateAverage([
      ffiBox.margin_top, ffiBox.margin_right, ffiBox.margin_bottom, ffiBox.margin_left
    ]),
    'padding': calculateAverage([
      ffiBox.padding_top, ffiBox.padding_right, ffiBox.padding_bottom, ffiBox.padding_left
    ]),
    'border_width': calculateAverage([
      ffiBox.border_width_top, ffiBox.border_width_right, ffiBox.border_width_bottom, ffiBox.border_width_left
    ]),
  };
} 