import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:jwt_decoder/jwt_decoder.dart';

final storage = FlutterSecureStorage();

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
      final token = jsonDecode(response.body);
      await storage.write(key: 'jwt', value: token);
      return token;
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
      final token = jsonDecode(response.body)['Ok'];
      await storage.write(key: 'jwt', value: token);
      return jsonDecode(response.body);
    } else if (response.statusCode == 401) {
      throw Exception('Unauthorized');
    } else if (response.statusCode == 500) {
      throw Exception('Internal server error');
    } else {
      throw Exception('Error');
    }
  }

  Future<void> signOut() async {
    final url = Uri.parse('$baseUrl/users/sign_out');
    final response = await http.delete(
      url,
      headers: {'Content-Type': 'application/json'},
    );
    if (response.statusCode == 200) {
      await storage.delete(key: 'jwt');
    } else if (response.statusCode == 401) {
      throw Exception('Unauthorized');
    } else if (response.statusCode == 500) {
      throw Exception('Internal server error');
    } else {
      throw Exception('Error');
    }
  }

  Future<Map<String, dynamic>> fetchUserProfile() async {
    final url = Uri.parse('$baseUrl/users/me');
    final response = await http.get(
      url,
      headers: {
        'Content-Type': 'application/json',
      },
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
}

void decodeJwt(String token) {
  List<String> parts = token.split('.');
  if (parts.length == 3) {
    String payload = parts[1];
    _base64UrlDecode(payload);
  } else {
    print("Token mal formé");
  }
}

String _base64UrlDecode(String input) {
  String output = input.replaceAll('-', '+').replaceAll('_', '/');
  return utf8.decode(base64Url.decode(output));
}

Future<bool> isTokenExpired() async {
  final token = await storage.read(key: 'jwt');

  if (token == null || token.isEmpty) {
    return true;
  }

  decodeJwt(token);

  try {
    final Map<String, dynamic> decodedToken = JwtDecoder.decode(token);

    if (decodedToken.containsKey("expiration")) {
      int expirationTime = int.parse(decodedToken["expiration"]);
      int currentTime = DateTime.now().millisecondsSinceEpoch ~/ 1000;

      return expirationTime < currentTime;
    }

    return true;
  } catch (e) {
    print("Erreur lors de la vérification du token : $e");
    return true;
  }
}

Future<String> getToken() async {
  final token = await storage.read(key: 'jwt');
  if (token == null) {
    throw Exception('Token not found');
  }
  return token;
}