import 'package:flutter/material.dart';
import 'browser_app.dart';

void logPrint(Object? obj) {
  // Optionally, also print to console
  // ignore: avoid_print
  print(obj);
}

void main() {
  logPrint('[DART] main() started');
  runApp(const BrowserApp());
  logPrint('[DART] main() finished');
}

