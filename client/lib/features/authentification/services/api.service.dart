import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiAccountService {
  final String baseUrl;

  ApiAccountService({required this.baseUrl});

  Future<String> signIn(String email, String password) async {
    final url = Uri.parse('$baseUrl/users/sign_in');
    final response = await http.post(
      url,
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'email': email, 'password': password}),
    );

    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else if (response.statusCode == 401) {
      throw Exception('Unauthorized');
    } else if (response.statusCode == 500) {
      throw Exception('Internal server error');
    } else {
      throw Exception('Error');
    }
  }

  Future<Map<String, dynamic>> signUp(String name, String email, String password, String passwordConfirmation) async {
    final url = Uri.parse('$baseUrl/users/sign_up');
    final response = await http.post(
      url,
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({
        'name': name,
        'email': email,
        'password': password,
        'password_confirmation': passwordConfirmation,
      }),
    );

    if (response.statusCode == 201) {
      return jsonDecode(response.body);
    } else if (response.statusCode == 409) {
      throw Exception('User already exists');
    } else if (response.statusCode == 400) {
      throw Exception('Invalid information');
    } else if (response.statusCode == 500) {
      throw Exception('Internal server error');
    } else {
      throw Exception('Error');
    }
  }

  Future<void> signOut(String token) async {
    final url = Uri.parse('$baseUrl/users/sign_out');
    final response = await http.post(
      url,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
    );

    if (response.statusCode != 200) {
      throw Exception('Failed to sign out');
    }
  }
}