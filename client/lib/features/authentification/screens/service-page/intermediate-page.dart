import 'package:client/features/authentification/screens/service-page/api-page-services.dart';
import 'package:client/features/authentification/screens/service-page/detail-page.dart';
import 'package:client/widgets/card-grid.dart';
import 'package:flutter/material.dart';

class IntermediatePage extends StatefulWidget {

  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  final int itemIndex;

  IntermediatePage({required this.itemIndex, super.key});

  @override
  State<IntermediatePage> createState() => _IntermediatePageState();
}

class _IntermediatePageState extends State<IntermediatePage> {
  final ApiService apiService = ApiService(baseUrl: IntermediatePage.baseUrlString);

  List<dynamic> cards = [
    {
      'title': 'Service 1',
      'description': 'Service 1 Description',
      'action': ['Action 1', 'Action 2'],
    },
    {
      'title': 'Service 2',
      'description': 'Service 2 Description',
      'action': ['Action 1', 'Action 2'],
    },
  ];

  @override
  void initState() {
    super.initState();
    fetchCards();
  }

  Future<void> fetchCards() async {
    try {
      final fetchedCards = await apiService.fetchCards(0);
      setState(() {
        cards = fetchedCards;
      });
    } catch (e) {
      print(e);
    }
  }

  void navigateToDetailPage(BuildContext context, dynamic card, int index) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => DetailPage(itemIndex: index, card: cards[index]),
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