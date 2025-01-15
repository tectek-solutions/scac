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
import 'package:flutter_cache_manager/flutter_cache_manager.dart';

const baseUrlString =
    String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
final storage = FlutterSecureStorage();

class ReactionPage extends StatefulWidget {
  const ReactionPage({super.key});

  @override
  _ReactionPageState createState() => _ReactionPageState();
}

class _ReactionPageState extends State<ReactionPage> {
  ApiService apiService = ApiService(
      baseUrl: IntermediatePageReaction.baseUrlString,
      route: '/authentications/');
  List<dynamic> services = [];

  Future<void> navigateToIntermediatePage(
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
      // url = url.replaceAll('"', '');
      //print("Attempting to launch URL: $url");
      if (await canLaunchUrlString(url)) {
        print("Launching URL: $url");
        await launchUrlString(url, mode: LaunchMode.externalApplication);
      } else {
        print("Failed to launch URL: $url");
        throw 'Could not launch $url';
      }
    } else {
      print("HERE IS THE RESPONSE: ${response.body}");
      // Navigator.push(
      //     context,
      //     MaterialPageRoute(
      //       builder: (context) => IntermediatePageReaction(itemIndex: index, id: 1),
      //     ),
      //   );
    }
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

  Future<void> clearAppCache() async {
    await DefaultCacheManager().emptyCache();
    print("App cache cleared");
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

  // @override
  // Widget build(BuildContext context) {
  //   return Scaffold(
  //     appBar: AppBar(
  //       title: Text('Choose a Service'),
  //       actions: [
  //         IconButton(
  //           icon: Icon(Icons.delete),
  //           onPressed: () async {
  //             await clearAppCache();
  //           },
  //         ),
  //       ],
  //     ),
  //     body: CardGrid(
  //       appBarTitle: 'Choose a Service',
  //       cards: services,
  //       icon: Icons.star_half,
  //       onTap: navigateToIntermediatePage,
  //     ),
  //   );
  // }
}
