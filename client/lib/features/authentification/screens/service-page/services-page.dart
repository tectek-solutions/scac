import 'package:client/features/authentification/screens/service-page/api-page-services.dart';
import 'package:client/widgets/card-grid.dart';
import 'package:flutter/material.dart';
import 'intermediate-page.dart'; // Import the new IntermediatePage

class ServicePage extends StatefulWidget {
  const ServicePage({super.key});

  @override
  _ServicePageState createState() => _ServicePageState();
}

class _ServicePageState extends State<ServicePage> {
  List<dynamic> services = [
    {
      'title': 'Google',
      'description': 'Google Description',
    },
    {
      'title': 'Microsoft',
      'description': 'Microsoft Description',
    },
  ];

  void navigateToIntermediatePage(BuildContext context, dynamic card, int index) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => IntermediatePage(itemIndex: index),
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