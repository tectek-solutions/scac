import 'package:client/features/authentification/screens/service-page/api-page-services.dart';
import 'package:flutter/material.dart';
import 'api-page-services.dart';

class ServicesPage extends StatefulWidget {
  @override
  _ServicesPageState createState() => _ServicesPageState();
}

class _ServicesPageState extends State<ServicesPage> {
  List<dynamic> cards = [
    {
      'title': 'Card 1',
      'action': 'Action 1',
      'description': 'Description 1',
    },
    {
      'title': 'Card 2',
      'action': 'Action 2',
      'description': 'Description 2',
    },
    {
      'title': 'Card 3',
      'action': 'Action 3',
      'description': 'Description 3',
    },
    {
      'title': 'Card 4',
      'action': 'Action 4',
      'description': 'Description 4',
    },
    {
      'title': 'Card 5',
      'action': 'Action 5',
      'description': 'Description 5',
    },
    {
      'title': 'Card 6',
      'action': 'Action 6',
      'description': 'Description 6',
    },
    {
      'title': 'Card 7',
      'action': 'Action 7',
      'description': 'Description 7',
    },
    {
      'title': 'Card 8',
      'action': 'Action 8',
      'description': 'Description 8',
    },
  ];
  final ApiService apiService = ApiService();

  @override
  void initState() {
    super.initState();
    fetchCards();
  }

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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Choose a Service'),
        centerTitle: true,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Expanded(
              child: GridView.builder(
                gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: 2,
                  crossAxisSpacing: 10.0,
                  mainAxisSpacing: 10.0,
                ),
                itemCount: cards.length,
                itemBuilder: (context, index) {
                  return GestureDetector(
                      onTap: () {
                        Navigator.push(
                          context,
                          _createRoute(DetailPage(itemIndex: index, card: cards[index])),
                        );
                      },
                      child: Card(
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(15.0),
                        ),
                        child: Center(
                          child: Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              const Icon(Icons.star_half, size: 30.0),
                              const SizedBox(height: 8.0),
                              Text(cards[index]['title'], style: TextStyle(fontSize: 15.0)),
                            ],
                          ),
                        ),
                      ),
                    );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }

  Route _createRoute(Widget page) {
    return PageRouteBuilder(
      pageBuilder: (context, animation, secondaryAnimation) => page,
      transitionsBuilder: (context, animation, secondaryAnimation, child) {
        const begin = Offset(1.0, 0.0);
        const end = Offset.zero;
        const curve = Curves.ease;

        var tween = Tween(begin: begin, end: end).chain(CurveTween(curve: curve));

        return SlideTransition(
          position: animation.drive(tween),
          child: child,
        );
      },
    );
  }
}

class DetailPage extends StatelessWidget {
  final int itemIndex;
  final dynamic card;

  DetailPage({required this.itemIndex, required this.card});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(card['title']),
      ),
      body: Center(
        child: Text('Detail Page for ${card['title']}'),
      ),
    );
  }
}