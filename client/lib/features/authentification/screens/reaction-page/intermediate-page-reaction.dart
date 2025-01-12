import 'package:client/features/authentification/screens/reaction-page/api-reaction-page.dart';
import 'package:client/features/authentification/screens/reaction-page/detail-page-reaction.dart';
import 'package:client/widgets/card-grid.dart';
import 'package:flutter/material.dart';

class IntermediatePageReaction extends StatefulWidget {

  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  int itemIndex;
  int id;

  IntermediatePageReaction({required this.itemIndex, required this.id, super.key});

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
        print("HERE IS THE VALUE $value");
      }
      print("Passed value: $value");
      setState(() {
        for (var i = 0; i < value.length; i++) {
          print("Value: ${value[i]['name']}");
          cards.add({'title': value[i]['name']});
          print("Cards: $cards");
        }
      });
    });
  }

  List<dynamic> cards = [];

  void navigateToDetailPage(BuildContext context, dynamic card, int index) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => DetailPage(itemIndex: index, id: widget.id, card: cards[index]),
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