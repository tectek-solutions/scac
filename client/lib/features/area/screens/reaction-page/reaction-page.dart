import 'dart:convert';
import 'package:client/features/services/api.area.service.dart';
import 'package:client/features/area/screens/reaction-page/intermediate-page-reaction.dart';
import 'package:client/features/area/screens/reaction-page/intermediate-page-reaction.dart';
import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:url_launcher/url_launcher_string.dart';
import '../../../../widgets/card-grid.dart';
import 'package:http/http.dart' as http;
import '../../../services/api.service.dart';

const baseUrlString =
    String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
final storage = FlutterSecureStorage();

class ReactionPage extends StatefulWidget {
  final dynamic action;
  const ReactionPage(this.action, {super.key});

  @override
  _ReactionPageState createState() => _ReactionPageState();
}

class _ReactionPageState extends State<ReactionPage> {
  ApiService apiService = ApiService(baseUrl: IntermediatePageReaction.baseUrlString, route: '/authentications/');
  List<dynamic> services = [];

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
        builder: (context) => IntermediatePageReaction(itemIndex: index, id: service['id'], action: widget.action),
      ),
    );
  }

  // void navigateToIntermediatePage(BuildContext context, dynamic card, int index) {
  //   Navigator.push(
  //     context,
  //     MaterialPageRoute(
  //       builder: (context) => IntermediatePageReaction(itemIndex: index, id: services[index]['id']),
  //     ),
  //   );
  // }

  _ReactionPageState() {
    apiService.fetchCards().then((value) {
      if (value is Map<String, dynamic>) {
        value = [value];
      }
      setState(() {
        for (var i = 0; i < value.length; i++) {
          services.add({
            'id': value[i]['id'],
            'title': value[i]['name'],
            'description': 'Use a ${value[i]['name']}\'s service.'
          });
        }
      });
    });
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
