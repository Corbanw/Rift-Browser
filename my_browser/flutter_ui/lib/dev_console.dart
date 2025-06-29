import 'package:flutter/material.dart';
import 'dart:async';
import 'dart:io';

class LogManager extends ChangeNotifier {
  static final LogManager _instance = LogManager._internal();
  factory LogManager() => _instance;
  LogManager._internal();

  static const int _maxLogLines = 300; // Further reduced from 500 to 300
  final List<String> _logs = [];
  List<String> get logs => List.unmodifiable(_logs.reversed);
  
  // Batch logging to reduce memory pressure
  final List<String> _pendingLogs = [];
  Timer? _batchTimer;
  static const Duration _batchInterval = Duration(milliseconds: 500);

  // Memory monitoring
  Timer? _memoryTimer;
  double _lastMemoryUsage = 0;

  void add(String log) {
    _pendingLogs.add('[${DateTime.now().toIso8601String()}] $log');
    
    // Start batch timer if not already running
    _batchTimer ??= Timer(_batchInterval, _processBatch);
  }

  void _processBatch() {
    _batchTimer = null;
    
    // Add all pending logs
    _logs.addAll(_pendingLogs);
    _pendingLogs.clear();
    
    // Trim to max size
    if (_logs.length > _maxLogLines) {
      final toRemove = _logs.length - _maxLogLines;
      _logs.removeRange(0, toRemove);
    }
    
    // Only notify every 20 logs to reduce rebuild frequency
    if (_logs.length % 20 == 0 || _logs.length <= 20) {
      notifyListeners();
    }
  }

  void startMemoryMonitoring() {
    _memoryTimer?.cancel();
    _memoryTimer = Timer.periodic(const Duration(seconds: 5), (timer) {
      _checkMemoryUsage();
    });
  }

  void stopMemoryMonitoring() {
    _memoryTimer?.cancel();
    _memoryTimer = null;
  }

  void _checkMemoryUsage() {
    try {
      if (Platform.isWindows) {
        // Get process memory info on Windows
        final process = Process.runSync('tasklist', ['/FI', 'IMAGENAME eq browser_ui.exe', '/FO', 'CSV']);
        if (process.exitCode == 0) {
          final lines = process.stdout.toString().split('\n');
          if (lines.length > 1) {
            final parts = lines[1].split(',');
            if (parts.length > 4) {
              final memoryStr = parts[4].replaceAll('"', '').replaceAll(' K', '');
              final memoryKb = double.tryParse(memoryStr) ?? 0;
              final memoryMb = memoryKb / 1024;
              
              if (_lastMemoryUsage > 0) {
                final diff = memoryMb - _lastMemoryUsage;
                if (diff > 50) { // Alert if memory increased by more than 50MB
                  add('[MEMORY] Memory increased by ${diff.toStringAsFixed(1)} MB (${memoryMb.toStringAsFixed(1)} MB total)');
                }
              }
              _lastMemoryUsage = memoryMb;
            }
          }
        }
      }
    } catch (e) {
      // Ignore memory monitoring errors
    }
  }

  void clear() {
    _logs.clear();
    _pendingLogs.clear();
    _batchTimer?.cancel();
    _batchTimer = null;
    notifyListeners();
  }

  @override
  void dispose() {
    _batchTimer?.cancel();
    _memoryTimer?.cancel();
    super.dispose();
  }
}

class DevConsole extends StatefulWidget {
  final VoidCallback onClose;
  static const int maxLogsToDisplay = 200; // Further reduced from 300 to 200
  const DevConsole({Key? key, required this.onClose}) : super(key: key);

  @override
  State<DevConsole> createState() => _DevConsoleState();
}

class _DevConsoleState extends State<DevConsole> {
  late ScrollController _scrollController;
  Timer? _updateTimer;
  String _cachedLogs = '';
  DateTime _lastUpdate = DateTime.now();

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
    
    // Start memory monitoring
    LogManager().startMemoryMonitoring();
    
    // Update the console every 3 seconds instead of 2 to reduce frequency
    _updateTimer = Timer.periodic(const Duration(seconds: 3), (timer) {
      if (mounted) {
        _updateLogs();
      }
    });
  }

  void _updateLogs() {
    final now = DateTime.now();
    // Only update if more than 2 seconds have passed or if logs have changed significantly
    if (now.difference(_lastUpdate).inSeconds >= 2) {
      final newLogs = LogManager().logs.take(DevConsole.maxLogsToDisplay).join('\n');
      if (newLogs != _cachedLogs) {
        setState(() {
          _cachedLogs = newLogs;
          _lastUpdate = now;
        });
      }
    }
  }

  @override
  void dispose() {
    _updateTimer?.cancel();
    _scrollController.dispose();
    // Stop memory monitoring when console is closed
    LogManager().stopMemoryMonitoring();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.black.withOpacity(0.85),
      child: SafeArea(
        child: SizedBox(
          width: MediaQuery.of(context).size.width * 0.5,
          child: Column(
            children: [
              Row(
                children: [
                  IconButton(
                    icon: const Icon(Icons.close, color: Colors.white),
                    onPressed: widget.onClose,
                  ),
                  const Spacer(),
                  IconButton(
                    icon: const Icon(Icons.delete, color: Colors.white),
                    onPressed: () => LogManager().clear(),
                    tooltip: 'Clear logs',
                  ),
                ],
              ),
              const Divider(color: Colors.white24, height: 1),
              Expanded(
                child: Container(
                  padding: const EdgeInsets.all(8),
                  child: SelectableText(
                    _cachedLogs,
                    style: const TextStyle(
                      color: Colors.white,
                      fontFamily: 'monospace',
                      fontSize: 11, // Even smaller font
                      height: 1.2, // Tighter line height
                    ),
                    scrollPhysics: const ClampingScrollPhysics(),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
} 