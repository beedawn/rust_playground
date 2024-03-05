import logo from './logo.svg';
import './App.css';
import axios from 'axios';
/*function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}*/


import React, { Component } from 'react';
class App extends Component {
state = {
"pending_items": [],
"done_items": [],
"pending_items_count": 0,
"done_items_count": 0
}

//Makes API call
getItems(){
axios.get("http://127.0.0.1:8000/v1/item/get",
{headers: {"token": "some_token"}})
.then(response => {
let pending_items = response.data["pending_items"]
let done_items = response.data["done_items"]
this.setState({
"pending_items": this.processItemValues(pending_items),
"done_items": this.processItemValues(done_items),
"pending_items_count": response.data["pending_item_count"],
"done_items_count": response.data["done_item_count"]
})
});

  }

  //ensures the API call is updated when mounted
  componentDidMount() {
this.getItems();
  }

  //convert items from API to HTML
  processItemValues(items){
let itemList = [];
items.forEach((item, index)=>{
itemList.push(
<li key={index}>{item.title} {item.status}</li>
)
})
return itemList
  }
render(){
return(

<div className="App">
<h1>Done Items</h1>
<p>done item count: {this.state.done_items_count}</p>
{this.state.done_items}
<h1>Pending Items</h1>
<p>pending item count:
{this.state.pending_items_count}</p>
{this.state.pending_items}
</div>



)

  }
}
export default App;
