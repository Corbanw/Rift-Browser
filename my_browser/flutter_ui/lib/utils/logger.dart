import 'dart:io';
import 'dart:convert';

enum LogLevel {
  debug,
  info,
  warning,
  error,
  performance
}

class Logger {
  static final Logger _instance = Logger._internal();
  factory Logger() => _instance;
  Logger._internal();

  static const bool _enableFileLogging = true;
  static const bool _enableConsoleLogging = true;
  static const int _maxLogLines = 1000;
  
  final List<String> _logs = [];
  File? _logFile;

  Future<void> init() async {
    if (_enableFileLogging) {
      try {
        final directory = Directory.current;
        _logFile = File('${directory.path}/browser_debug.log');
        await _logFile!.writeAsString('Browser Debug Log - ${DateTime.now()}\n');
      } catch (e) {
        print('Failed to initialize log file: $e');
      }
    }
  }

  void log(LogLevel level, String component, String message, [Map<String, dynamic>? data]) {
    final timestamp = DateTime.now().toIso8601String();
    final levelStr = level.name.toUpperCase().padRight(11);
    final componentStr = component.padRight(15);
    
    String logEntry = '[$timestamp] $levelStr [$componentStr] $message';
    
    if (data != null && data.isNotEmpty) {
      try {
        final dataStr = jsonEncode(data);
        logEntry += ' | Data: $dataStr';
      } catch (e) {
        logEntry += ' | Data: ${data.toString()}';
      }
    }

    // Add to memory logs
    _logs.add(logEntry);
    if (_logs.length > _maxLogLines) {
      _logs.removeAt(0);
    }

    // Console logging
    if (_enableConsoleLogging) {
      print(logEntry);
    }

    // File logging
    if (_enableFileLogging && _logFile != null) {
      try {
        _logFile!.writeAsStringSync('$logEntry\n', mode: FileMode.append);
      } catch (e) {
        // Fail silently to avoid infinite loops
      }
    }
  }

  void debug(String component, String message, [Map<String, dynamic>? data]) {
    log(LogLevel.debug, component, message, data);
  }

  void info(String component, String message, [Map<String, dynamic>? data]) {
    log(LogLevel.info, component, message, data);
  }

  void warning(String component, String message, [Map<String, dynamic>? data]) {
    log(LogLevel.warning, component, message, data);
  }

  void error(String component, String message, [Map<String, dynamic>? data]) {
    log(LogLevel.error, component, message, data);
  }

  void performance(String component, String operation, int durationMs, [Map<String, dynamic>? data]) {
    final perfData = {
      'operation': operation,
      'duration_ms': durationMs,
      ...?data,
    };
    log(LogLevel.performance, component, 'Performance: $operation took ${durationMs}ms', perfData);
  }

  List<String> getLogs() => List.unmodifiable(_logs);
  
  void clearLogs() {
    _logs.clear();
  }

  // Performance measurement helper
  Future<T> measurePerformance<T>(String component, String operation, Future<T> Function() function) async {
    final stopwatch = Stopwatch()..start();
    try {
      debug(component, 'Starting $operation');
      final result = await function();
      stopwatch.stop();
      performance(component, operation, stopwatch.elapsedMilliseconds);
      return result;
    } catch (e) {
      stopwatch.stop();
      error(component, 'Failed $operation after ${stopwatch.elapsedMilliseconds}ms: $e');
      rethrow;
    }
  }

  T measureSync<T>(String component, String operation, T Function() function) {
    final stopwatch = Stopwatch()..start();
    try {
      debug(component, 'Starting $operation (sync)');
      final result = function();
      stopwatch.stop();
      performance(component, operation, stopwatch.elapsedMilliseconds);
      return result;
    } catch (e) {
      stopwatch.stop();
      error(component, 'Failed $operation after ${stopwatch.elapsedMilliseconds}ms: $e');
      rethrow;
    }
  }
}
