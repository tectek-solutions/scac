import 'package:client/features/authentification/screens/service-page/api-page-services.dart';
import 'package:client/features/authentification/screens/service-page/intermediate-page.dart';
import 'package:flutter/material.dart';

class DetailPage extends StatefulWidget {
  final int itemIndex;
  final dynamic card;

  const DetailPage({required this.itemIndex, required this.card, Key? key}) : super(key: key);

  @override
  State<DetailPage> createState() => _DetailPageState();
}

class _DetailPageState extends State<DetailPage> {

  final ApiService apiService = ApiService(baseUrl: IntermediatePage.baseUrlString, route: '/actions/1');
  List<dynamic> actions = [];

  _DetailPageState() {
    apiService.fetchCards().then((value) {
      if (value is Map<String, dynamic>) {
        value = [value];
        print("HERE IS THE VALUE $value");
      }
      print("Passed value: $value");
      setState(() {
        for (var i = 0; i < value.length; i++) {
          print("Value: ${value[i]['name']}");
          actions.add(value[i]['name']);
          print("Actions: $actions");
        }
      });
    });
  }

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
                      actions[widget.itemIndex],
                      style: const TextStyle(fontSize: 22.0, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 10.0),
                    const Text(
                      'No Description',
                      style: TextStyle(fontSize: 16.0),
                    ),
                  ],
                ),
              ),
              const Divider(),
              Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: actions.map<Widget>((action) {
                  return ElevatedButton(
                    onPressed: () {
                      int count = 0;
                      Navigator.of(context).popUntil((route) {
                        count++;
                        if (count == 3) { 
                          //Voila la data qui est envoyée
                          Navigator.pop(context, {
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
