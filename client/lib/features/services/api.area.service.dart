import 'package:http/http.dart' as http;
import 'dart:convert';
import 'api.service.dart';

class ApiService {
  final String baseUrl;
  final String route;

  ApiService({required this.baseUrl, required this.route});

  Future<dynamic> fetchCards() async {
    final url = Uri.parse('$baseUrl$route');
    final token = getToken();
    print(token);
    final response = await http.get(
      url,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
    );
    if (response.statusCode == 200) {
      final dynamic data = jsonDecode(response.body);
      return data;
    } else {
      throw Exception('Failed to fetch');
    }
  }
}
