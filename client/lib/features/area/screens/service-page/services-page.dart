import 'dart:io';
import 'package:client/features/area/screens/reaction-page/reaction-page.dart';
import 'package:client/features/services/api.area.service.dart';
import 'package:client/widgets/card-grid.dart';
import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import 'intermediate-page.dart';
import 'package:http/http.dart' as http;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ServicePage extends StatefulWidget {
  const ServicePage({super.key});

  @override
  _ServicePageState createState() => _ServicePageState();
}

class _ServicePageState extends State<ServicePage> {
  ApiService apiService = ApiService(baseUrl: IntermediatePage.baseUrlString, route: '/authentications/');
  List<dynamic> services = [];
  final storage = FlutterSecureStorage();

  _ServicePageState() {
    apiService.fetchCards().then((value) {
      if (value is Map<String, dynamic>) {
        value = [value];
      }
      setState(() {
        for (var i = 0; i < value.length; i++) {
          services.add({'id': value[i]['id'], 'title': value[i]['name'], 'description': 'Use a ${value[i]['name']}\'s service.'});
        }
      });
    });
  }

  Future<void> navigateToIntermediatePage (
    BuildContext context, dynamic card, int index) async {
    final token = await storage.read(key: 'jwt');
    final service = services[index];
    final response = await http.get(
      Uri.parse('$baseUrlString/user-tokens/authentications/${service['id']}'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
    );
    if (response.statusCode != 200) {
      final response = await http.get(
        Uri.parse(
            '$baseUrlString/user-tokens/url/authentications/${service['id']}'),
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer $token',
        },
      );
      var url = response.body.substring(1, response.body.length - 1);
      await launchUrl(Uri.parse(url), mode: LaunchMode.inAppBrowserView);
    }
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => IntermediatePage(itemIndex: index, id: service['id']),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return CardGrid(
      appBarTitle: 'Choose a Service',
      cards: services,
      icon: Icons.star_half,
      onTap: navigateToIntermediatePage,
    );
  }
}