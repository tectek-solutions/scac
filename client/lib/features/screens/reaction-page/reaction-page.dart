import 'package:client/features/authentification/screens/service-page/api-page-services.dart';
import 'package:client/features/authentification/screens/service-page/intermediate-page.dart';
import 'package:flutter/material.dart';
import '../../../../widgets/card-grid.dart';

class ReactionPage extends StatefulWidget {
  @override
  _ReactionPageState createState() => _ReactionPageState();
}

class _ReactionPageState extends State<ReactionPage> {
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
        builder: (context) => IntermediatePage(itemIndex: index, id: 1,),
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