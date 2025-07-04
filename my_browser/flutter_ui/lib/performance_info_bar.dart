import 'package:flutter/material.dart';

// TODO: Move warning logic to its own helper if needed.

class PerformanceInfoBar extends StatelessWidget {
  final String currentHtml;
  final String currentCss;
  final String scrollOffset;
  final bool isDarkMode;

  const PerformanceInfoBar({
    super.key,
    required this.currentHtml,
    required this.currentCss,
    required this.scrollOffset,
    required this.isDarkMode,
  });

  @override
  Widget build(BuildContext context) {
    final hasWarnings = currentHtml.length > 100000; // Simple warning based on HTML size
    return Container(
      padding: const EdgeInsets.all(8.0),
      color: hasWarnings ? Colors.orange[50] : Colors.grey[100],
      child: Row(
        children: [
          Icon(
            hasWarnings ? Icons.warning : Icons.info_outline,
            size: 16,
            color: hasWarnings ? Colors.orange[700] : Colors.grey[600],
          ),
          const SizedBox(width: 8),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  'Paint Renderer: \\${currentHtml.length} chars HTML, \\${currentCss.length} chars CSS',
                  style: TextStyle(
                    fontSize: 12,
                    color: hasWarnings ? Colors.orange[700] : Colors.grey[600],
                    fontWeight: hasWarnings ? FontWeight.bold : FontWeight.normal,
                  ),
                ),
                if (hasWarnings)
                  Text(
                    '⚠️ Large page - rendering may be slow',
                    style: TextStyle(
                      fontSize: 10,
                      color: Colors.orange[700],
                      fontStyle: FontStyle.italic,
                    ),
                  ),
              ],
            ),
          ),
          Text(
            'Scroll: \\${scrollOffset}',
            style: TextStyle(fontSize: 12, color: Colors.grey[600]),
          ),
        ],
      ),
    );
  }
} 