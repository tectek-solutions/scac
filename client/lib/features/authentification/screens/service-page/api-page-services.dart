import 'package:http/http.dart' as http;
import 'dart:convert';

class ApiService {
  final String baseUrl;
  final String route;

  ApiService({required this.baseUrl, required this.route});

  Future<dynamic> fetchCards() async {
    print("Fetching cards from $baseUrl$route");
    final url = Uri.parse('$baseUrl$route');
    final response = await http.get(
      url,
      headers: {'Content-Type': 'application/json'},
    );
    if (response.statusCode == 200) {
      final dynamic data = jsonDecode(response.body);
      return data;
    } else {
      throw Exception('Failed to load cards');
    }
  }
}