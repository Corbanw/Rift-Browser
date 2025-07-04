import 'package:http/http.dart' as http;

Future<String> fetchHtml(String url, void Function(String) logPrint) async {
  logPrint('[DART] _fetchHtml called with url: $url');
  final fullUrl = ensureUrlHasProtocol(url);
  logPrint('[DART] _fetchHtml: Fetching from $fullUrl');
  try {
    final response = await http.get(Uri.parse(fullUrl));
    return handleHttpResponse(response, logPrint);
  } catch (e) {
    handleNetworkError(e, logPrint);
    rethrow;
  }
}

Future<String> fetchExternalCss(String url, void Function(String) logPrint) async {
  try {
    final response = await http.get(Uri.parse(url));
    if (response.statusCode == 200) {
      return response.body;
    } else {
      logPrint('[DART] Failed to fetch external CSS: $url (${response.statusCode})');
      return '';
    }
  } catch (e) {
    logPrint('[DART] Error fetching external CSS: $url ($e)');
    return '';
  }
}

String handleHttpResponse(http.Response response, void Function(String) logPrint) {
  if (response.statusCode == 200) {
    logPrint('[DART] _fetchHtml: Successfully fetched \\${response.body.length} characters');
    return response.body;
  } else {
    logPrint('[ERROR] _fetchHtml: HTTP \\${response.statusCode}');
    throw Exception('HTTP \\${response.statusCode}: \\${response.reasonPhrase}');
  }
}

void handleNetworkError(dynamic error, void Function(String) logPrint) {
  logPrint('[ERROR] _fetchHtml: Network error: $error');
  throw Exception('Failed to fetch HTML: $error');
}

String ensureUrlHasProtocol(String url) {
  if (!url.startsWith('http://') && !url.startsWith('https://')) {
    return 'https://$url';
  }
  return url;
}

String resolveUrl(String href, String baseUrl) {
  if (href.startsWith('http')) return href;
  try {
    final base = Uri.parse(baseUrl);
    return Uri.parse(href).isAbsolute ? href : base.resolve(href).toString();
  } catch (_) {
    return href;
  }
} 