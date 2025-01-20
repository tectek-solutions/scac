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

  Future<void> removeCard(int id) async {
    final url = Uri.parse('$baseUrl$route$id');
    final token = await storage.read(key: 'jwt');
    final response = await http.delete(
      url,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
    );
    if (response.statusCode == 200) {
      return;
    } else {
      throw Exception('Failed to delete');
    }
  }

  Future<bool> addCard(String name, String description, int actionId, int reactionId, Map<String, String> actionData, Map<String, String> reactionData) async {
    final url = Uri.parse('$baseUrl$route');
    final token = await storage.read(key: 'jwt');
    if (name == "" || description == "" || actionId == 0 || reactionId == 0) {
      throw Exception('Failed to add');
    }
    final response = await http.post(
      url,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
      body: jsonEncode({
        'name': name,
        'users_id': 0,
        'description': description,
        'actions_id': actionId,
        'reactions_id': reactionId,
        'action_data': actionData,
        'reaction_data': reactionData,
      }),
    );
    if (response.statusCode == 200) {
      return true;
    } else {
      throw Exception('Failed to add');
    }
  }

  Future<http.Response> downloadFile(String url) async {
    final response = await http.get(Uri.parse(url));
    return response;

  }
}
