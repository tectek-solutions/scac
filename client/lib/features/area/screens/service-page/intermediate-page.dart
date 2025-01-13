import 'package:client/features/services/api.area.service.dart';
import 'package:client/features/area/screens/service-page/detail-page.dart';
import 'package:client/widgets/card-grid.dart';
import 'package:flutter/material.dart';

class IntermediatePage extends StatefulWidget {

  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  int itemIndex;
  int id;

  IntermediatePage({required this.itemIndex, required this.id, super.key});

  @override
  State<IntermediatePage> createState() => _IntermediatePageState();
}

class _IntermediatePageState extends State<IntermediatePage> {
  late final ApiService apiService;

  @override
  void initState() {
    super.initState();
    apiService = ApiService(baseUrl: IntermediatePage.baseUrlString, route: '/apis/${widget.id}');
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