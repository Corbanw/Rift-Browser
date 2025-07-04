String extractTextContent(String html) {
  try {
    String text = html;
    text = text.replaceAll(RegExp(r'<script[^>]*>.*?</script>', dotAll: true), '');
    text = text.replaceAll(RegExp(r'<style[^>]*>.*?</style>', dotAll: true), '');
    text = text.replaceAll(RegExp(r'<[^>]*>'), '');
    text = text.replaceAll('&amp;', '&');
    text = text.replaceAll('&lt;', '<');
    text = text.replaceAll('&gt;', '>');
    text = text.replaceAll('&quot;', '"');
    text = text.replaceAll('&#39;', "'");
    text = text.replaceAll('&nbsp;', ' ');
    text = text.replaceAll(RegExp(r'\s+'), ' ').trim();
    return text;
  } catch (e) {
    return 'Content extraction failed';
  }
} 