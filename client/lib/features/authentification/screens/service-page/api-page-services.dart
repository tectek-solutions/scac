import 'package:http/http.dart' as http;
import 'dart:convert';

class ApiService {
  final String baseUrl;
  // final String token;

  ApiService({required this.baseUrl});

  Future<List<dynamic>> fetchCards(int id) async {
    final url = Uri.parse('$baseUrl/authentications/$id');
    final response = await http.get(
      url,
      headers: {'Content-Type': 'application/json'},
    );
    if (response.statusCode == 200) {
      final List<dynamic> cards = jsonDecode(response.body);
      return cards;
    } else {
      throw Exception('Failed to load cards');
    }
  }
}