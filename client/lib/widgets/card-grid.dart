import 'package:flutter/material.dart';

class CardGrid extends StatelessWidget {
  final String appBarTitle;
  final List<dynamic> cards;
  final IconData icon;
  final void Function(BuildContext context, dynamic card, int index) onTap;

  const CardGrid({
    required this.appBarTitle,
    required this.cards,
    required this.icon,
    required this.onTap,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(appBarTitle),
        centerTitle: true,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: GridView.builder(
          gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 2,
            crossAxisSpacing: 10.0,
            mainAxisSpacing: 10.0,
          ),
          itemCount: cards.length,
          itemBuilder: (context, index) {
            final card = cards[index];
            return GestureDetector(
              onTap: () => onTap(context, card, index),
              child: Card(
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(15.0),
                ),
                child: Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(icon, size: 30.0),
                      const SizedBox(height: 8.0),
                      Text(
                        card['title'] ?? '',
                        style: const TextStyle(fontSize: 15.0),
                      ),
                    ],
                  ),
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
