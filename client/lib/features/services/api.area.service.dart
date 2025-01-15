import 'package:http/http.dart' as http;
import 'dart:convert';
import 'api.service.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

final storage = const FlutterSecureStorage();

class ApiService {
  final String baseUrl;
  final String route;

  ApiService({required this.baseUrl, required this.route});

  Future<dynamic> fetchCards() async {
    final url = Uri.parse('$baseUrl$route');
    final token = await storage.read(key: 'jwt');
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
