import 'dart:io';
import 'package:client/features/services/api.area.service.dart';
import 'package:client/widgets/card-grid.dart';
import 'package:flutter/material.dart';
import 'intermediate-page.dart';

class ServicePage extends StatefulWidget {
  const ServicePage({super.key});

  @override
  _ServicePageState createState() => _ServicePageState();
}

class _ServicePageState extends State<ServicePage> {
  ApiService apiService = ApiService(baseUrl: IntermediatePage.baseUrlString, route: '/authentications/');
  List<dynamic> services = [];

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

  void navigateToIntermediatePage(BuildContext context, dynamic card, int index) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => IntermediatePage(itemIndex: index, id: services[index]['id']),
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