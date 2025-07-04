import 'dart:io';

void logMemoryUsage(String context, void Function(String) logPrint) {
  try {
    final memoryInfo = ProcessInfo.currentRss;
    logPrint('[MEMORY] $context - RSS: \\${(memoryInfo / 1024 / 1024).toStringAsFixed(2)} MB');
  } catch (e) {
    logPrint('[MEMORY] $context - Unable to get memory info: $e');
  }
} 