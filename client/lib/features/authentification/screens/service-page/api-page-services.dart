import 'package:http/http.dart' as http;
import 'dart:convert';

class ApiService {
  Future<List<dynamic>> fetchCards() async {
    final response = await http.get(Uri.parse('https://api.example.com/cards'));

    if (response.statusCode == 200) {
      return json.decode(response.body);
    } else {
      throw Exception('Failed to load cards');
    }
  }
}