from django.shortcuts import render
from django.http import HttpResponse

# Create your views here.

def index(request):
    meetups = [
        
        {'title': 'A first Meetup', 
        'location': 'New York', 
        'slug': 'a-first-meetup'},
        
        {'title': 'A second Meetup', 
        'location': 'Paris', 
        'slug': 'a-first-meetup'},
        
        {'title': 'A third Meetup', 
        'location': 'Singapore', 
        'slug': 'a-first-meetup'}
    ]
    return render(request, 'meetups/index.html', {
        'meetups': meetups,
        'show_meetups': True
    })

def meetup_details(request, meetup_slug):
    selected_meetup = {
        'title': 'A First Meetup', 
        'description': 'This is the First Meetup!'
        }
    return render(request, 'meetups/meetup-details.html', {
        'meetup_title': selected_meetup['title'],
        'meetup_description': selected_meetup['description']
    })