import 'dart:convert';
import 'package:client/features/authentification/screens/reaction-page/api-reaction-page.dart';
import 'package:client/features/authentification/screens/reaction-page/intermediate-page-reaction.dart';
import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../../../../widgets/card-grid.dart';
import 'package:http/http.dart' as http;

const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');

class ReactionPage extends StatefulWidget {
  const ReactionPage({super.key});

  @override
  _ReactionPageState createState() => _ReactionPageState();
}

class _ReactionPageState extends State<ReactionPage> {
  ApiService apiService = ApiService(baseUrl: IntermediatePageReaction.baseUrlString, route: '/authentications/');
  List<dynamic> services = [];

  // Future<void> navigateToIntermediatePage(BuildContext context, dynamic card, int index) async {
  //   final service = services[index];
  //   //remplacer par le bon call back url
  //   final response = await http.get(Uri.parse('$baseUrlString/user-tokens/authentications/${service['title'].toLowerCase()}'));

  //   if (response.statusCode == 200) {
  //     final result = jsonDecode(response.body);
  //     if (result['hasToken']) {
  //       Navigator.push(
  //         context,
  //         MaterialPageRoute(
  //           builder: (context) => IntermediatePage(itemIndex: index, id: 1),
  //         ),
  //       );
  //     } else {
  //       final authUrl = service['authUrl'];
  //       if (await canLaunchUrl(authUrl)) {
  //         await launchUrl(authUrl);
  //       } else {
  //         throw 'Could not launch $authUrl';
  //       }
  //     }
  //   } else {
  //     throw 'Failed to check token';
  //   }
  // }

  void navigateToIntermediatePage(BuildContext context, dynamic card, int index) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => IntermediatePageReaction(itemIndex: index, id: services[index]['id']),
      ),
    );
  }

  _ReactionPageState() {
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