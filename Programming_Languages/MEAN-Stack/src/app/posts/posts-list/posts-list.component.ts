import { Component, Input } from "@angular/core";


@Component({
  selector: 'app-post-list',
  templateUrl: './posts-list.component.html',
  styleUrls: ['./posts-list.component.css']
})
export class PostListComponent{
  //posts = [
  //  {title: 'First Post', content: 'This is the first post\'s content'},
  // {title: 'Second Post', content: 'This is the second post\'s content'},
  //  {title: 'Third Post', content: 'This is the third post\'s content'}
  //];
  @Input() posts = [];
}
