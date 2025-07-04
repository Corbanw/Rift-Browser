import 'dart:async';

typedef SetStateCallback = void Function(void Function());

enum LoadErrorType {
  timeout,
  network,
  notFound,
  forbidden,
  server,
  ssl,
  other,
}

String categorizeError(dynamic error) {
  String errorMessage = error.toString();
  if (error is TimeoutException) {
    return 'Request timeout - the page took too long to load';
  } else if (errorMessage.contains('SocketException')) {
    return 'Network error - unable to connect to the server';
  } else if (errorMessage.contains('404')) {
    return 'Page not found (404) - the requested page does not exist';
  } else if (errorMessage.contains('403')) {
    return 'Access forbidden (403) - you do not have permission to access this page';
  } else if (errorMessage.contains('500')) {
    return 'Server error (500) - the server encountered an internal error';
  } else if (errorMessage.contains('SSL') || errorMessage.contains('certificate')) {
    return 'Security error - SSL certificate verification failed';
  }
  return errorMessage;
}

void handleLoadError({
  required SetStateCallback setState,
  required void Function(String) logPrint,
  required dynamic error,
  required String url,
  required void Function(String) setErrorMessage,
  required void Function(String) setCurrentUrl,
}) {
  logPrint('[DART] _loadPage: Error: $error');
  final errorMessage = categorizeError(error);
  setState(() {
    setErrorMessage(errorMessage);
    setCurrentUrl(url);
  });
} 