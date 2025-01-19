import 'dart:convert';
import 'package:client/features/area/screens/reaction-page/reaction-page.dart';
import 'package:client/features/services/api.area.service.dart';
import 'package:client/features/area/screens/reaction-page/detail-page-reaction.dart';
import 'package:client/widgets/card-grid.dart';
import 'package:flutter/material.dart';

class IntermediatePageReaction extends StatefulWidget {

  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  int itemIndex;
  int id;
  final dynamic action;

  IntermediatePageReaction({required this.itemIndex, required this.id, required this.action, super.key});

  @override
  State<IntermediatePageReaction> createState() => _IntermediatePageReactionState();
}

class _IntermediatePageReactionState extends State<IntermediatePageReaction> {
  late final ApiService apiService;

  @override
  void initState() {
    super.initState();
    apiService = ApiService(baseUrl: IntermediatePageReaction.baseUrlString, route: '/apis/${widget.id}');
    apiService.fetchCards().then((value) {
      if (value is Map<String, dynamic>) {
        value = [value];
      }
      setState(() {
        for (var i = 0; i < value.length; i++) {
          cards.add({'title': value[i]['name']});
        }
      });
    });
  }

  List<dynamic> cards = [];

  void navigateToDetailPage(BuildContext context, dynamic card, int index) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => DetailPage(itemIndex: index, id: widget.id, card: cards[index], action: widget.action),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return CardGrid(
      appBarTitle: 'Choose an Action',
      cards: cards,
      icon: Icons.star_half,
      onTap: navigateToDetailPage,
    );
  }
}