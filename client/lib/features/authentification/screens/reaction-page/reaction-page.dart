import 'package:client/features/authentification/screens/service-page/api-page-services.dart';
import 'package:flutter/material.dart';
import '../../../../widgets/card-grid.dart';

class ReactionPage extends StatefulWidget {
  @override
  _ReactionPageState createState() => _ReactionPageState();
}

class _ReactionPageState extends State<ReactionPage> {
  // Exemple de comment doivent etre les donnees
  List<dynamic> cards = [
    {
      'title': 'Reaction 1',
      'description': 'Reaction 1 Description',
      'action': ['Action 1', 'Action 2'],
    },
    {
      'title': 'Reaction 2',
      'description': 'Reaction 2 Description',
      'action': ['Action 1', 'Action 2'],
    },
    {
      'title': 'Reaction 3',
      'description': 'Reaction 3 Description',
      'action': ['Action 1', 'Action 2'],
    },
  ];
  // Initialisation du service API
  final ApiService apiService = ApiService();

  @override
  void initState() {
    super.initState();
    fetchCards();
  }

  //fonction pour recuperer les cartes
  Future<void> fetchCards() async {
    try {
      final fetchedCards = await apiService.fetchCards();
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
        builder: (context) => DetailPage(itemIndex: index, card: card),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return CardGrid(
      appBarTitle: 'Choose a Reaction',
      cards: cards,
      icon: Icons.star_half,
      onTap: navigateToDetailPage,
    );
  }
}

class DetailPage extends StatelessWidget {
  final int itemIndex;
  final dynamic card;

  const DetailPage({required this.itemIndex, required this.card, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Detail Page'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Card(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(15.0),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      card['title'] ?? 'No Title',
                      style: const TextStyle(fontSize: 22.0, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 10.0),
                    Text(
                      card['description'] ?? 'No Description',
                      style: const TextStyle(fontSize: 16.0),
                    ),
                  ],
                ),
              ),
              const Divider(),
              Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: (card['action'] as List).map<Widget>((action) {
                    return ElevatedButton(
                      onPressed: () {
                        int count = 0;
                        Navigator.of(context).popUntil((route) {
                          count++;
                          if (count == 2) { 
                            //Voila la data qui est envoyée
                            Navigator.pop(context, {
                              'title': card['title'],
                              'description': card['description'],
                              'action': action,
                            });
                            return true;
                          }
                          return false;
                        });
                        //Debug
                        print('$action button pressed');
                      },
                      child: Text(action),
                    );
                  }).toList(),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}