import 'package:flutter/material.dart';
import 'browser_screen.dart';

class BrowserApp extends StatelessWidget {
  const BrowserApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Rift Browser',
      theme: _createAppTheme(),
      home: const BrowserScreen(),
    );
  }

  ThemeData _createAppTheme() {
    return ThemeData(
      primarySwatch: Colors.blue,
      useMaterial3: true,
      brightness: Brightness.light,
      scaffoldBackgroundColor: Colors.white,
      appBarTheme: const AppBarTheme(
        backgroundColor: Colors.blue,
        foregroundColor: Colors.white,
        elevation: 2,
      ),
    );
  }
} 