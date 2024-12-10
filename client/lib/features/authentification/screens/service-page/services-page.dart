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
      'action': [
        'Action 1',
        'Action 2',
        'Action 3',
        'Action 4',
      ],
      'description': 'Description 1',
    },
    {
      'title': 'Card 2',
      'action': [
        'Action 1',
        'Action 2',
        'Action 3',
        'Action 4',
      ],
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
        title: Text('Detail Page'),
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
                child: card['action'] is List
                    ? Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: (card['action'] as List).map<Widget>((action) {
                          return ElevatedButton(
                            onPressed: () {
                              int count = 0;
                              Navigator.of(context).popUntil((route) {
                                count++;
                                if (count == 2) { // Adjust count based on how many levels you want to pop
                                  Navigator.pop(context, card['title']); // Pass data back
                                  return true;
                                }
                                return false;
                              });
                              print('$action button pressed');
                            },
                            child: Text(action),
                          );
                        }).toList(),
                      )
                    : ElevatedButton(
                        onPressed: () {
                          // Define the action for the button here
                          print('${card['action']} button pressed2');
                        },
                        child: Text(card['action'] ?? 'No Action'),
                      ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
