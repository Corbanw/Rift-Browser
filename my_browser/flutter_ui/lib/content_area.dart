import 'package:flutter/material.dart';

// TODO: Move error page, loading indicator, and page content widgets to their own files if needed.

class ContentArea extends StatelessWidget {
  final bool isLoading;
  final bool hasError;
  final String errorMessage;
  final bool isDarkMode;
  final String currentHtml;
  final String currentCss;
  final Widget Function()? buildErrorPage;
  final Widget Function()? buildLoadingContent;
  final Widget Function()? buildEmptyContent;
  final Widget Function()? buildPageContent;

  const ContentArea({
    super.key,
    required this.isLoading,
    required this.hasError,
    required this.errorMessage,
    required this.isDarkMode,
    required this.currentHtml,
    required this.currentCss,
    this.buildErrorPage,
    this.buildLoadingContent,
    this.buildEmptyContent,
    this.buildPageContent,
  });

  @override
  Widget build(BuildContext context) {
    if (hasError && buildErrorPage != null) {
      return buildErrorPage!();
    }
    if (isLoading && buildLoadingContent != null) {
      return buildLoadingContent!();
    }
    if (currentHtml.isEmpty && buildEmptyContent != null) {
      return buildEmptyContent!();
    }
    if (buildPageContent != null) {
      return buildPageContent!();
    }
    return const SizedBox.shrink();
  }
} 