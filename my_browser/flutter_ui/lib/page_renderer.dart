import 'package:flutter/material.dart';
import 'paint_renderer.dart';

// TODO: Move fallback renderer and error widget logic to their own files if needed.

class PageRenderer extends StatelessWidget {
  final String currentHtml;
  final String currentCss;
  final ScrollController scrollController;
  final bool isDarkMode;
  final double Function()? getScrollOffsetValue;

  const PageRenderer({
    super.key,
    required this.currentHtml,
    required this.currentCss,
    required this.scrollController,
    required this.isDarkMode,
    this.getScrollOffsetValue,
  });

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        try {
          return SingleChildScrollView(
            controller: scrollController,
            child: PaintRenderer(
              html: currentHtml,
              css: currentCss,
              scrollOffset: getScrollOffsetValue != null ? getScrollOffsetValue!() : 0.0,
              viewportSize: Size(constraints.maxWidth, constraints.maxHeight),
              isDarkMode: isDarkMode,
              onRenderComplete: () {},
            ),
          );
        } catch (e) {
          // TODO: Use a fallback renderer or error widget
          return const Center(
            child: Text('Failed to render page content'),
          );
        }
      },
    );
  }
} 