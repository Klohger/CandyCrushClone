using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class swag : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        Debug.Log(Newtonsoft.Json.JsonConvert.SerializeObject(Stage.testStage, new Newtonsoft.Json.JsonSerializerSettings() { Formatting = Newtonsoft.Json.Formatting.Indented, TypeNameHandling = Newtonsoft.Json.TypeNameHandling.Objects, TypeNameAssemblyFormatHandling = Newtonsoft.Json.TypeNameAssemblyFormatHandling.Simple}));
    }
}
