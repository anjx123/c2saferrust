from .openai import OpenAIGen, OpenAIEmbed, OpenAIReasoning
from .google import GoogleGen
from .claude import ClaudeGen
from dotenv import load_dotenv

load_dotenv()

class ModelException(Exception):
    pass


def get_model_from_name(name):

    if name == "gpt4":
        return OpenAIGen(model="gpt-4-0125-preview")
    elif name == "o3-mini":
        return OpenAIReasoning(model="o3-mini")
    elif name == "gpt4o":
        return OpenAIGen(model="gpt-4o-2024-05-13")
    elif name == "gpt4o-mini":
        return OpenAIGen(model="gpt-4o-mini-2024-07-18")
    elif name == "gpt3":
        return OpenAIGen(model="gpt-3.5-turbo")
    elif name == "gemini":
        return GoogleGen(model="gemini-2.5-flash-preview-04-17")
    elif name == "claude":
        return ClaudeGen(model="claude-3-opus-20240229")
    elif name == "embedding":
        return OpenAIEmbed(model="text-embedding-3-large")
    else:
        raise NotImplementedError("Unknown model name")


__all__ = [
    "OpenAIGen",
    "GoogleGen",
    "ClaudeGen",
    "OpenAIEmbed",
    "ModelException",
    "get_model_from_name",
]
