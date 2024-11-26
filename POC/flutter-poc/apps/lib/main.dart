import 'dart:convert';
import 'dart:developer';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;

List<PostList> list = [];
Future<List<PostList>> getProductData() async {
  http.Response response =
  await http.get(Uri.parse("https://jsonplaceholder.typicode.com/posts"));
  if (response.statusCode == 200) {
    try {
      var data = jsonDecode(response.body) as List;
      list = data.map((e) => PostList.fromJson(e)).toList();
    } catch (e) {
      log("message", error: e.toString());
    }
  }
  return list;
}

  void main() {
    WidgetsFlutterBinding.ensureInitialized();
    runApp(const MyApp());
  }

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: HomePage(),
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Rest Api Demo Flutter"),
      ),
      body: FutureBuilder(
          future: getProductData(),
          builder: (context, AsyncSnapshot<List<PostList>> postSnapshot) {
            if (postSnapshot.hasData) {
              return ListView.builder(
                  itemCount: postSnapshot.data!.length,
                  itemBuilder: (context, index) {
                    return ListTile(
                      title: Text(postSnapshot.data![index].title,
                          style: const TextStyle(
                            fontWeight: FontWeight.w500,
                            fontSize: 20,
                          )),
                      subtitle: Text(postSnapshot.data![index].body),
                    );
                  });
            }
            return const Center(
              child: CircularProgressIndicator(),
            );
          }),
    );
  }
}

class PostList {
  int userId;
  int id;
  String title;
  String body;

  PostList(
      {required this.id,
        required this.userId,
        required this.title,
        required this.body});

  factory PostList.fromJson(dynamic json) => PostList(
      id: json["id"],
      userId: json["userId"],
      title: json["title"],
      body: json["body"]);
}