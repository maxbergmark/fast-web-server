import time
from locust import FastHttpUser, task, between

class QuickstartUser(FastHttpUser):
    # wait_time = between(1, 5)

    @task
    def test(self):
        self.client.get("/test")

