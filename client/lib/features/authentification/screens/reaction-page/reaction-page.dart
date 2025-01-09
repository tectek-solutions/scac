import 'dart:convert';

import 'package:client/features/authentification/screens/service-page/api-page-services.dart';
import 'package:client/features/authentification/screens/service-page/intermediate-page.dart';
import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../../../../widgets/card-grid.dart';
import 'package:http/http.dart' as http;

const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');

class ReactionPage extends StatefulWidget {
  @override
  _ReactionPageState createState() => _ReactionPageState();
}

class _ReactionPageState extends State<ReactionPage> {
  List<dynamic> services = [
    {
      'title': 'Google',
      'description': 'Google Description',
      //remplacer par le bon call back url
      'authUrl': 'https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id=936038757007-d2vvj4kjm98vcod9e9ek9ilvoeij1fcr.apps.googleusercontent.com&redirect_uri=$baseUrlString/oauth2/authorize/google&scope=https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/calendar https://mail.google.com/&state=test_state',
    },
    {
      'title': 'Microsoft',
      'description': 'Microsoft Description',
      //remplacer par le bon call back url
      'authUrl': 'https://login.microsoftonline.com/common/oauth2/v2.0/authorize?response_type=code&client_id=3e226b46-9ef1-42bf-a557-a73ca86aed7c&redirect_uri=$baseUrlString/oauth2/authorize/microsoft&scope=email openid profile offline_access User.Read Mail.Read Mail.ReadWrite Mail.Send&state=test_state',
    },
  ];

  Future<void> navigateToIntermediatePage(BuildContext context, dynamic card, int index) async {
    final service = services[index];
    //remplacer par le bon call back url
    final response = await http.get(Uri.parse('$baseUrlString/api/check-token?service=${service['title']}'));

    if (response.statusCode == 200) {
      final result = jsonDecode(response.body);
      if (result['hasToken']) {
        Navigator.push(
          context,
          MaterialPageRoute(
            builder: (context) => IntermediatePage(itemIndex: index, id: 1),
          ),
        );
      } else {
        final authUrl = service['authUrl'];
        if (await canLaunchUrl(authUrl)) {
          await launchUrl(authUrl);
        } else {
          throw 'Could not launch $authUrl';
        }
      }
    } else {
      throw 'Failed to check token';
    }
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