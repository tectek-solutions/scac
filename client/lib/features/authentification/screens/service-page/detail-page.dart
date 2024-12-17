import 'package:flutter/material.dart';

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
                          if (count == 3) { 
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
